#[macro_use]
mod macros;

pub mod game;
pub mod state;
pub mod draw;
pub mod score;
pub mod rest;
pub mod err;

mod tetromino;
mod pos;
mod board;
mod piece;
mod game_over;

#[macro_use]
extern crate lazy_static;

extern crate sdl2;
extern crate rand;
extern crate rustc_serialize;
extern crate regex;

#[cfg(not(target_os = "emscripten"))]
extern crate hyper;

#[cfg(target_os = "emscripten")]
extern crate emscripten;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
