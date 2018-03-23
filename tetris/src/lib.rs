#[macro_use]
mod macros;

pub mod game;
pub mod state;
pub mod score;
pub mod rest;
pub mod err;
pub mod game_over;
pub mod tetromino;
pub mod board;
pub mod piece;
pub mod pos;

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

use state::State;
use state::StateChange;

pub struct Tetris {
    states: Vec<State>
}

impl Default for Tetris {
    fn default() -> Self {
        let mut states = Vec::new();
        states.push(State::Title);
        Tetris { states }
    }
}

impl Tetris {
    pub fn state(&mut self) -> &mut State {
        self.states.last_mut().unwrap()
    }

    pub fn apply_state_change(&mut self, state_change: StateChange) {
        state_change.apply(&mut self.states);
    }
}
