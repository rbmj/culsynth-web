#!/bin/bash
set -e
cd `dirname $0`
mkdir -p static
wasm-pack build --target web --no-typescript --out-dir ../static/pkg culsynth-web-audioworklet
wasm-pack build --target web --no-typescript --out-dir ../static/pkg culsynth-web-ui
