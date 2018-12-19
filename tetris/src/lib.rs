pub use self::board::Board;
pub use self::game::{Game, GameWithHistory};
pub use self::game_over::{GameOver, HighScores};
pub use self::piece::Piece;
pub use self::pos::Pos;
pub use self::score::{Score, ScoreMessage, SCORE_ENDPOINT};
pub use self::shape::{Rotation, Shape, ShapeColor};
pub use self::state::{Paused, State, Title};

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
