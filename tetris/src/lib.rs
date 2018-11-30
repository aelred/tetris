#[macro_use]
mod macros;

pub mod args;
pub mod board;
pub mod game;
pub mod game_over;
pub mod piece;
pub mod pos;
pub mod rest;
pub mod score;
pub mod shape;
pub mod state;

#[macro_use]
extern crate lazy_static;

extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate hyper;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
