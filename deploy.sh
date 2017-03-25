#!/bin/bash

# fail immediately if a command fails
set -e
set -o pipefail

cargo build --target asmjs-unknown-emscripten --release

cp target/asmjs-unknown-emscripten/release/tetris.js static/tetris.js

scp -r static felix@ael.red:~/sites/tetris/
