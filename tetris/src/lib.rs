//! Library representing a Tetris game.
//! ```
//! use tetris::State;
//! use tetris::State::*;
//!
//! // Initialise a game at the title screen
//! let mut state = State::title();
//!
//! loop {
//!     // Match on the state of the game and get a new state back.
//!     state = match state {
//!         Title(title) => {
//!             title.start_game()
//!         },
//!         Play(ref mut game) => {
//!             // Play some random moves
//!             game.move_left();
//!             game.rotate();
//!             game.start_hard_drop();
//!             state
//!         }
//!         Paused(paused) => {
//!             paused.unpause()
//!         }
//!         GameOver(mut game_over) => {
//!             // Submit my high-score!
//!             game_over.push_name("BOB");
//!             game_over.submit()
//!         }
//!     };
//!
//!     // Update the state of the game by one tick
//!     state = state.update();
//!
//!     // Draw game state and sleep
//!     // ...
//!     # break;
//! }
//! ```

pub use self::board::Board;
pub use self::game::Game;
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
