#!/bin/bash

# fail immediately if a command fails
set -e
set -o pipefail

cd tetris-sdl
cargo build --release --target wasm32-unknown-emscripten
cd ..
cp target/wasm32-unknown-emscripten/release/tetris-sdl.js static/
cp target/wasm32-unknown-emscripten/release/tetris_sdl.wasm static/
