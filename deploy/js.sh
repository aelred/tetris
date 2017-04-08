#!/bin/bash
source /opt/emsdk-portable/emsdk_env.sh
cargo build --bin tetris --target asmjs-unknown-emscripten --release
cp target/asmjs-unknown-emscripten/release/tetris.js static/tetris.js

