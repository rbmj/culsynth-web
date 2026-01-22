#!/bin/bash
set -e
cd `dirname $0`
mkdir -p static
cargo build
wasm-bindgen --keep-debug --target web --no-typescript --out-dir ./static/pkg target/wasm32-unknown-unknown/debug/culsynth_web_audioworklet.wasm
wasm-bindgen --keep-debug --target web --no-typescript --out-dir ./static/pkg target/wasm32-unknown-unknown/debug/culsynth_web_ui.wasm
echo '*** Build Complete - Artifacts are in ./static ***'
