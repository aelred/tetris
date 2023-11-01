# Tetris

Play it [here](http://tetris.ael.red/).

## Build

### Local

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
just run
```

Or install it with:
```sh
just install
```

### Browser

To build the browser version, you need to [install Emscripten 3.1.43](https://emscripten.org/docs/getting_started/downloads.html#installation-instructions-using-the-emsdk-recommended), then run:

```sh
just build-wasm
```

After this, the files will all be in `static/`:
