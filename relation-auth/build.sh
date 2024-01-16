#!/usr/bin/env bash
set -euo pipefail
if ! command -v ic-wasm
then
    echo could not find ic-cdk-optimizer
    echo "ic-wasm is needed, please run the following command:"
    echo "  cargo install ic-wasm"
    exit 1
fi

AUTH_DIR="$(dirname "$0")"
TARGET="wasm32-unknown-unknown"
cargo_build_args=(
    --manifest-path "$AUTH_DIR/Cargo.toml"
    --target "$TARGET"
    --release
    -j1
    )
echo Running cargo build "${cargo_build_args[@]}"
cargo build "${cargo_build_args[@]}"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$AUTH_DIR/target}"

ic-wasm \
    "$CARGO_TARGET_DIR/$TARGET/release/relation_auth.wasm" \
    -o "$CARGO_TARGET_DIR/$TARGET/release/relation_auth-opt.wasm" \
    optimize O3
