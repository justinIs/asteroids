#!/usr/bin/env bash
# Upload the built web dist/ folder and the Android APK to S3.
#
# Usage:
#   ./deploy.sh                       # uses AWS_PROFILE from deploy.env (or env)
#   ./deploy.sh --profile my-profile  # override the profile
#   DEPLOY_CONFIG=/path/to/env ./deploy.sh
#
# Build the web assets first with ./build-web.sh, and (optionally) the APK with
# ./build-android.sh. The APK is uploaded only if it has been built.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_FILE="${DEPLOY_CONFIG:-$SCRIPT_DIR/deploy.env}"

# Settings that aren't secret stay here; the rest come from the config file.
CACHE_CONTROL="max-age=3600"
DIST_DIR="dist"
APK_SRC="target/android-artifacts/release/apk/asteroids.apk"

# Optional --profile flag overrides whatever the config / environment sets.
PROFILE_OVERRIDE=""
case "${1:-}" in
--profile | -p)
    if [[ -z "${2:-}" ]]; then
        echo "error: --profile requires a value" >&2
        exit 1
    fi
    PROFILE_OVERRIDE="$2"
    ;;
"") ;;
*)
    echo "unknown argument: $1" >&2
    echo "usage: $0 [--profile <aws-profile>]" >&2
    exit 1
    ;;
esac

# Load deployment target from the config file. It must define BUCKET, PREFIX and
# (optionally) AWS_PROFILE.
if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "error: config file not found: $CONFIG_FILE" >&2
    echo "Copy deploy.env.example to deploy.env and fill in your values." >&2
    exit 1
fi
# shellcheck source=/dev/null
source "$CONFIG_FILE"

# Flag wins over the config file's AWS_PROFILE.
if [[ -n "$PROFILE_OVERRIDE" ]]; then
    AWS_PROFILE="$PROFILE_OVERRIDE"
fi

if [[ -z "${BUCKET:-}" || -z "${PREFIX:-}" ]]; then
    echo "error: BUCKET and PREFIX must be set in $CONFIG_FILE" >&2
    exit 1
fi

if [[ -z "${AWS_PROFILE:-}" ]]; then
    echo "error: no AWS profile set. Set AWS_PROFILE in $CONFIG_FILE or pass --profile <name>." >&2
    exit 1
fi
export AWS_PROFILE

if [[ ! -d "$DIST_DIR" ]]; then
    echo "error: $DIST_DIR/ not found. Run ./build-web.sh first." >&2
    exit 1
fi

DEST="s3://$BUCKET/$PREFIX"
echo "==> Uploading $DIST_DIR/ to $DEST (profile: $AWS_PROFILE)"

# Sync everything except the wasm and apk, letting the CLI guess content types.
# Excluding them also protects them from --delete (e.g. a web-only deploy won't
# wipe a previously uploaded APK).
aws s3 sync "$DIST_DIR/" "$DEST" \
    --cache-control "$CACHE_CONTROL" \
    --exclude "*.wasm" \
    --exclude "asteroids.apk" \
    --delete

# Upload the wasm separately so we can force the correct content type;
# browsers need application/wasm to stream-instantiate the module.
aws s3 cp "$DIST_DIR/asteroids.wasm" "${DEST}asteroids.wasm" \
    --cache-control "$CACHE_CONTROL" \
    --content-type "application/wasm"

# Upload the Android APK to the same prefix, if it has been built.
if [[ -f "$APK_SRC" ]]; then
    echo "==> Uploading $APK_SRC"
    aws s3 cp "$APK_SRC" "${DEST}asteroids.apk" \
        --cache-control "$CACHE_CONTROL" \
        --content-type "application/vnd.android.package-archive"
else
    echo "==> Skipping APK (not found at $APK_SRC; run ./build-android.sh to include it)"
fi

echo "==> Done."
