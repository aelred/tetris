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

use rand;

use serde_json;

use hyper;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
