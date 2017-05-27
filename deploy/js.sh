#!/bin/bash
cargo build --bin tetris --target asmjs-unknown-emscripten --release
cp target/asmjs-unknown-emscripten/release/tetris.js static/tetris.js

