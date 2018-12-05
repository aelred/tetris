#[macro_use]
mod macros;

mod args;
mod board;
mod game;
mod game_over;
mod piece;
mod pos;
mod rest;
mod score;
mod shape;
mod state;

pub use board::Board;
pub use game::{Game, GameWithHistory};
pub use game_over::{GameOver, HighScores};
pub use piece::Piece;
pub use pos::Pos;
pub use score::{Score, ScoreMessage, SCORE_ENDPOINT};
pub use shape::{Shape, ShapeColor, Rotation};
pub use state::{State, Title, Paused};

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
