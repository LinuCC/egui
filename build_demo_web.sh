#!/bin/bash
set -eu

CRATE_NAME="egui_demo_app"

# This is required to enable the web_sys clipboard API which egui_web uses
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis

# Clear output from old stuff:
rm -f docs/${CRATE_NAME}_bg.wasm

echo "Building rust…"
BUILD=release
cargo build --release -p ${CRATE_NAME} --lib --target wasm32-unknown-unknown

echo "Generating JS bindings for wasm…"
TARGET_NAME="${CRATE_NAME}.wasm"
wasm-bindgen "target/wasm32-unknown-unknown/$BUILD/$TARGET_NAME" \
  --out-dir docs --no-modules --no-typescript

echo "Finished: docs/${CRATE_NAME}.wasm"

open http://localhost:8888/index.html
