#!/bin/bash
cd tetris-sdl
cargo build --release --target asmjs-unknown-emscripten
cd ..
cp target/asmjs-unknown-emscripten/release/tetris.js static/tetris.js
