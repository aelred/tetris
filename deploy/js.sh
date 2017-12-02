#!/bin/bash
cargo build --bin tetris --release --target asmjs-unknown-emscripten
cp target/asmjs-unknown-emscripten/release/tetris.js static/tetris.js
