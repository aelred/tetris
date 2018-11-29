# Tetris

Play it [here](http://tetris.ael.red/).

## Build

To build you'll need [Rust](http://www.rust-lang.org/) and [SDL2](https://www.libsdl.org/index.php).

To install SDL2 on Ubuntu:
```sh
sudo apt-get install libsdl2-dev libsdl2-ttf-dev libsdl2-mixer-dev
```

On Mac OSX with Brew:
```sh
brew install sdl2 sdl2_ttf sdl2_mixer
```

Then you can run the game with:
```sh
cargo run --bin tetris-sdl
```

Or install it with:
```sh
cargo install --path tetris-sdl
tetris-sdl
```
