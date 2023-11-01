install:
    cargo install --path tetris-sdl

init-wasm:
    rustup target add wasm32-unknown-emscripten

build-wasm: init-wasm
    cd tetris-sdl && cargo build --release --target wasm32-unknown-emscripten
    cp target/wasm32-unknown-emscripten/release/tetris_sdl.wasm static/
    cp target/wasm32-unknown-emscripten/release/tetris-sdl.js static/

run:
    cargo run --bin tetris-sdl

serve: build-wasm
    cargo run --bin tetris-server

test:
    cargo test