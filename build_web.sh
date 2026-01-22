#!/bin/bash
set -e
cd `dirname $0`
mkdir -p static
rm -rf static/pkg
wasm-pack build --release --target web --no-typescript --out-dir ../static/pkg culsynth-web-audioworklet
wasm-pack build --release --target web --no-typescript --out-dir ../static/pkg culsynth-web-ui
echo '*** Build Complete - Artifacts are in ./static ***'
