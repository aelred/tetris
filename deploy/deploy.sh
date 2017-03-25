cd ~/sites/tetris

git reset --hard
git pull

cargo build --target asmjs-unknown-emscripten --release

cp target/asmjs-unknown-emscripten/release/tetris.js static/tetris.js
