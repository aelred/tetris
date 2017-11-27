#!/bin/bash
wargo build --bin tetris --release
cp target/wasm32-unknown-emscripten/release/tetris.js static/tetris.js
cp target/wasm32-unknown-emscripten/release/deps/tetris-*.wasm static/
