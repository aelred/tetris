#!/bin/bash

# fail immediately if a command fails
set -e
set -o pipefail

cd ~/sites/tetris

source ~/scratch/emsdk-portable/emsdk_env.sh

git pull

cargo build --target asmjs-unknown-emscripten --release

cp target/asmjs-unknown-emscripten/release/tetris.js static/tetris.js
