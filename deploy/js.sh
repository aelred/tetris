#!/bin/bash

# fail immediately if a command fails
set -e
set -o pipefail

cd tetris-sdl
cargo build --release --target asmjs-unknown-emscripten
cd ..
cp target/asmjs-unknown-emscripten/release/tetris-sdl.js static/tetris.js
