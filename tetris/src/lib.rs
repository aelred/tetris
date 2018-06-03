#[macro_use]
mod macros;

pub mod board;
pub mod err;
pub mod game;
pub mod game_over;
pub mod piece;
pub mod pos;
pub mod rest;
pub mod score;
pub mod state;
pub mod tetromino;

#[macro_use]
extern crate lazy_static;

extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

#[cfg(not(target_os = "emscripten"))]
extern crate hyper;

#[cfg(target_os = "emscripten")]
extern crate libc;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
