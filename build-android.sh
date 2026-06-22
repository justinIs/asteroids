#!/usr/bin/env bash
# Build an Android APK for the game using a locally-built cargo-quad-apk image.
#
# Usage:
#   ./build-android.sh                 # release APK (builds the image first if missing)
#   ./build-android.sh --debug         # debug APK (faster compile, unoptimized)
#   ./build-android.sh --rebuild-image # force-rebuild the docker image, then build APK
#
# Requirements:
#   - docker OR podman (the first image build downloads ~GBs of Android SDK/NDK)
#
# Output:
#   target/android-artifacts/<profile>/apk/asteroids.apk
#
# Install it on a connected device (USB debugging on) with:
#   adb install -r target/android-artifacts/release/apk/asteroids.apk

set -euo pipefail

CRATE="asteroids"
IMAGE="asteroids-cargo-apk:local"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKERFILE="$SCRIPT_DIR/android/Dockerfile"

PROFILE_DIR="release"
APK_FLAGS=(--release)
REBUILD_IMAGE=0

# Parse flags (order-independent, repeatable).
for arg in "$@"; do
    case "$arg" in
    --debug | -d)
        PROFILE_DIR="debug"
        APK_FLAGS=()
        ;;
    --rebuild-image)
        REBUILD_IMAGE=1
        ;;
    *)
        echo "unknown argument: $arg" >&2
        echo "usage: $0 [--debug|-d] [--rebuild-image]" >&2
        exit 1
        ;;
    esac
done

# Pick a container engine: prefer docker, fall back to podman (Fedora's default).
# Override with: ENGINE=podman ./build-android.sh
if [[ -n "${ENGINE:-}" ]]; then
    :
elif command -v docker >/dev/null 2>&1; then
    ENGINE="docker"
elif command -v podman >/dev/null 2>&1; then
    ENGINE="podman"
else
    echo "error: need docker or podman on PATH (found neither)" >&2
    exit 1
fi

if [[ ! -f "$DOCKERFILE" ]]; then
    echo "error: Dockerfile not found at $DOCKERFILE" >&2
    exit 1
fi

# Build the image if it doesn't exist yet, or if --rebuild-image was passed.
if [[ "$REBUILD_IMAGE" -eq 1 ]] || ! "$ENGINE" image inspect "$IMAGE" >/dev/null 2>&1; then
    echo "==> Building $ENGINE image $IMAGE (one-time; downloads Android SDK/NDK)"
    "$ENGINE" build -t "$IMAGE" "$(dirname "$DOCKERFILE")"
fi

echo "==> Building $CRATE APK ($PROFILE_DIR) via $IMAGE ($ENGINE)"
# cargo-quad-apk injects miniquad's Android glue into a wrapper that is compiled
# under this crate's edition 2024. That glue uses the pre-2024 `#[no_mangle]`,
# which edition 2024 rejects (it must be `#[unsafe(no_mangle)]`). We can't edit
# the generated wrapper, so inside the container we: build once (which extracts
# miniquad's source into the cargo registry and then fails on the attribute),
# rewrite the bare attribute in that source, then build again (reusing the
# already-compiled dependencies, so only the wrapper recompiles).
"$ENGINE" run --rm \
    -e APK_FLAGS="${APK_FLAGS[*]}" \
    -v "$SCRIPT_DIR":/root/src:z \
    -w /root/src \
    "$IMAGE" \
    bash -euc '
        cargo quad-apk build $APK_FLAGS || true
        find "${CARGO_HOME:-/usr/local/cargo}/registry/src" \
            -path "*/miniquad-*/src/native/android/mod_inject.rs" \
            -exec sed -i "s/#\[no_mangle\]/#[unsafe(no_mangle)]/g" {} +
        cargo quad-apk build $APK_FLAGS
    '

APK_SRC="$SCRIPT_DIR/target/android-artifacts/$PROFILE_DIR/apk/$CRATE.apk"
if [[ ! -f "$APK_SRC" ]]; then
    echo "error: expected APK not found at $APK_SRC" >&2
    exit 1
fi

echo "==> Done:"
ls -lh "$APK_SRC"

cat <<EOF

Install on a connected device (USB debugging enabled) with:
  adb install -r $APK_SRC
EOF
