#!/bin/bash

# fail immediately if a command fails
set -e
set -o pipefail

cd ~/sites/tetris

source ~/scratch/emsdk-portable/emsdk_env.sh

git fetch --all
git reset --hard origin/master

cargo build --target asmjs-unknown-emscripten --release

cp target/asmjs-unknown-emscripten/release/tetris.js static/tetris.js

cd tetris-server
cargo install --force
sudo restart tetris
