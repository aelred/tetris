#!/bin/bash

cargo build --target asmjs-unknown-emscripten --release

cp target/asmjs-unknown-emscripten/release/tetris.js static/tetris.js

scp -r static felix@ael.red:~/sites/tetris/
