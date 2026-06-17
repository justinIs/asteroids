#!/usr/bin/env bash
# Build the game for the browser (wasm32) and assemble a servable dist/ folder.
#
# Usage:
#   ./build-web.sh            # debug build (faster compile, larger wasm)
#   ./build-web.sh --release  # optimized build (slower compile, smaller wasm)
#
# After building, serve dist/ over HTTP
#   npx serve dist

set -euo pipefail

CRATE="asteroids"
TARGET="wasm32-unknown-unknown"
PROFILE_DIR="debug"
CARGO_FLAGS=()

# Parse a single optional --release / -r flag.
case "${1:-}" in
--release | -r)
    CARGO_FLAGS+=(--release)
    PROFILE_DIR="release"
    ;;
"")
    ;;
*)
    echo "unknown argument: $1" >&2
    echo "usage: $0 [--release|-r]" >&2
    exit 1
    ;;
esac

echo "==> Building $CRATE for $TARGET ($PROFILE_DIR)"
cargo build --target "$TARGET" "${CARGO_FLAGS[@]}"

WASM_SRC="target/$TARGET/$PROFILE_DIR/$CRATE.wasm"
if [[ ! -f "$WASM_SRC" ]]; then
    echo "error: expected wasm not found at $WASM_SRC" >&2
    exit 1
fi

echo "==> Assembling dist/"
mkdir -p dist
cp index.html dist/index.html
cp "$WASM_SRC" "dist/$CRATE.wasm"

echo "==> Done. dist/ contains:"
ls -lh dist

cat <<'EOF'

Serve it with:
  npx serve dist

then open the printed http://localhost URL.
EOF
