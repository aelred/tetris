use std::cmp;
use std::ops::Add;
use std::ops::Mul;

use rand;
use rand::SeedableRng;
use rand::XorShiftRng;
use serde_derive::{Deserialize, Serialize};

use crate::board::Board;
use crate::board::FillResult;
use crate::game_over::GameOver;
use crate::piece::Piece;
use crate::shape::Bag;
use crate::state::Paused;
use crate::state::State;
use crate::Shape;

/// The rate at which pieces fall, measured in hundredths of cells per frame.
///
/// Typically notated with the suffix _G_, e.g. _3G_ means 3 cells per frame.
///
/// e.g. `Gravity(10)` = _0.1G_ = 0.1 cells per frame = 6 cells per second (at 60fps)
#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Gravity(u32);

impl Gravity {
    /// How many gravity units we subdivide a cell into - impacts the rate pieces fall.
    const UNITS_PER_CELL: u32 = 100;

    /// The initial gravity at the start of the game.
    const INITIAL: Gravity = Gravity(4);

    /// The gravity when doing a faster soft drop - defined as _1G_.
    const SOFT_DROP: Gravity = Gravity(Gravity::UNITS_PER_CELL);

    /// The gravity when doing a hard drop - is meant to appear instantaneous, so defined as _20G_.
    const HARD_DROP: Gravity = Gravity(Gravity::UNITS_PER_CELL * 20);

    /// The rate gravity increases every level.
    const INCREASE_PER_LEVEL: Gravity = Gravity(2);
}

impl Add for Gravity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Gravity(self.0 + rhs.0)
    }
}

impl Mul<u32> for Gravity {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Gravity(self.0 * rhs)
    }
}

/// The number of lines to clear to go to the next level - resulting in gravity increasing.
const NUM_LINES_CLEARED_PER_LEVEL: u32 = 10;

/// The different ways a piece can drop, depending on user input.
enum Drop {
    /// The normal drop rate, where gravity is based on the current level.
    Normal,
    /// The fast drop rate, where gravity is fixed to 1G.
    Soft,
    /// The immediate drop, where gravity is 20G.
    Hard,
}

/// Actions that a user can take in the game.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum Action {
    /// Move the piece to the left.
    MoveLeft,
    /// Move the piece to the right.
    MoveRight,
    /// Rotate the piece clockwise.
    Rotate,
    /// Start a fast soft drop.
    StartSoftDrop,
    /// Immediately drop and lock the piece.
    StartHardDrop,
    /// Stop a soft or hard drop and return to the normal drop speed.
    StopDrop,
}

/// Describes how much time has passed in frames.
#[derive(Serialize, Deserialize, Clone, Copy, PartialOrd, PartialEq, Debug)]
struct Tick(u32);

impl Tick {
    /// Create a new `Tick` at 0.
    fn new() -> Tick {
        Tick(0)
    }

    /// Advance forward one frame in time.
    fn incr(&mut self) {
        self.0 += 1;
    }
}

/// A game, bundled with a record of every action performed in that game.
///
/// This is used for live games, so the history can be sent to the server and used to verify the
/// score.
pub struct Game {
    game_state: Box<GameState>,
    history: History,
}

impl Default for Game {
    /// Create a new game with a random seed and empty history.
    fn default() -> Self {
        let seed = rand::random();
        Game {
            game_state: Box::new(GameState::new(seed)),
            history: History::new(seed),
        }
    }
}

impl Game {
    /// Get the current piece that the user is placing.
    pub fn piece(&self) -> &Piece {
        &self.game_state.piece
    }

    /// Get the board, made up of blocks from old pieces.
    pub fn board(&self) -> &Board {
        &self.game_state.board
    }

    /// Get the next shape that will be played
    pub fn next_shape(&self) -> Shape {
        self.game_state.bag.peek()
    }

    /// Get the number of lines that have been cleared.
    pub fn lines_cleared(&self) -> u32 {
        self.game_state.lines_cleared
    }

    /// Get the player's score.
    pub fn score(&self) -> u32 {
        self.game_state.score
    }

    /// Advance the game one frame.
    ///
    /// Consumes the game and returns the new state. In the event of a game over, the returned state
    /// will be a "game over" state, otherwise it will be the game itself.
    pub fn update(mut self) -> State {
        match self.game_state.apply_step() {
            StepResult::GameOver => {
                let game_over = GameOver::new(self.game_state.score, self.history.clone());
                State::GameOver(game_over)
            }
            StepResult::Continue => State::Play(self),
        }
    }

    /// Move the piece to the left.
    pub fn move_left(&mut self) {
        self.apply_action(Action::MoveLeft);
    }

    /// Move the piece to the right.
    pub fn move_right(&mut self) {
        self.apply_action(Action::MoveRight);
    }

    /// Rotate the piece clockwise.
    pub fn rotate(&mut self) {
        self.apply_action(Action::Rotate);
    }

    /// Start a fast soft drop.
    pub fn start_soft_drop(&mut self) {
        self.apply_action(Action::StartSoftDrop);
    }

    /// Immediately drop and lock the piece.
    pub fn start_hard_drop(&mut self) {
        self.apply_action(Action::StartHardDrop);
    }

    /// Stop a soft or hard drop and return to the normal drop speed.
    pub fn stop_drop(&mut self) {
        self.apply_action(Action::StopDrop);
    }

    /// Pause the game, consuming the current state and returning a "paused" state.
    pub fn pause(self) -> State {
        State::Paused(Paused(self))
    }

    /// Apply the given action to the game and record it in the history.
    fn apply_action(&mut self, action: Action) {
        self.history.push_action(self.game_state.tick, action);
        self.game_state.apply_action(action);
    }
}

/// A game of Tetris in-progress.
struct GameState {
    /// The current piece that the user is placing.
    piece: Piece,

    /// The board, made up of blocks from old pieces.
    board: Board,

    /// A bag to pull new pieces from.
    bag: Bag,

    /// How far the piece has dropped through the current cell - once it reaches 100 the piece
    /// drops one cell, or locks.
    ///
    /// If lock delay is on, this resets to zero when the piece is moved or rotated.
    drop_tick: u32,

    /// Toggled on when the piece can't drop down a cell. When the piece drops again, it will lock.
    lock_delay: bool,

    /// The current drop rate.
    drop: Drop,

    /// The number of lines that have been cleared.
    lines_cleared: u32,

    /// The player's score.
    score: u32,

    /// Number of frames since the game has started.
    tick: Tick,
}

impl GameState {
    /// Create a new game from the given seed, which determines the order pieces appear.
    fn new(seed: [u32; 4]) -> GameState {
        let mut bag = Bag::new(XorShiftRng::from_seed(seed));
        GameState {
            piece: Piece::new(bag.pop()),
            board: Board::default(),
            bag,
            drop_tick: 0,
            lock_delay: false,
            drop: Drop::Normal,
            lines_cleared: 0,
            score: 0,
            tick: Tick::new(),
        }
    }

    /// Apply the given action to the game.
    fn apply_action(&mut self, action: Action) {
        match action {
            Action::MoveLeft => {
                self.try_move_left();
            }
            Action::MoveRight => {
                self.try_move_right();
            }
            Action::Rotate => {
                self.try_rotate();
            }
            Action::StartSoftDrop => self.drop = Drop::Soft,
            Action::StartHardDrop => self.drop = Drop::Hard,
            Action::StopDrop => self.drop = Drop::Normal,
        }
    }

    /// Advance the game one frame. Returns whether this is a game over.
    fn apply_step(&mut self) -> StepResult {
        self.tick.incr();

        while self.drop_tick >= Gravity::UNITS_PER_CELL {
            self.drop_tick -= Gravity::UNITS_PER_CELL;
            if self.drop_piece() == StepResult::GameOver {
                return StepResult::GameOver;
            }
        }

        self.drop_tick += match self.drop {
            Drop::Normal => self.normal_gravity(),
            Drop::Soft => Gravity::SOFT_DROP,
            Drop::Hard => Gravity::HARD_DROP,
        }
        .0;

        StepResult::Continue
    }

    /// Get the normal gravity rate, based on the current level.
    fn normal_gravity(&self) -> Gravity {
        let level = self.lines_cleared / NUM_LINES_CLEARED_PER_LEVEL;

        let g = Gravity::INITIAL + Gravity::INCREASE_PER_LEVEL * level;

        // Normal gravity should never be faster than a soft drop.
        cmp::min(g, Gravity::SOFT_DROP)
    }

    /// Try to rotate the piece clockwise, including a wall-kick.
    ///
    /// Returns whether the rotation was successful.
    fn try_rotate(&mut self) -> bool {
        self.piece.rotate_clockwise();
        self.reset_lock_delay();

        let successful_rotation = !self.piece_overlaps_board() || self.try_wall_kick();

        if !successful_rotation {
            self.piece.rotate_anticlockwise();
        }

        successful_rotation
    }

    /// Try to move the piece left. Returns whether the piece was moved successfully.
    fn try_move_left(&mut self) -> bool {
        self.piece.left();
        self.reset_lock_delay();

        let collides = self.piece_overlaps_board();

        if collides {
            self.piece.right();
        }

        !collides
    }

    /// Try to move the piece right. Returns whether the piece was moved successfully.
    fn try_move_right(&mut self) -> bool {
        self.piece.right();
        self.reset_lock_delay();

        let collides = self.piece_overlaps_board();

        if collides {
            self.piece.left();
        }

        !collides
    }

    /// Reset the lock delay, if lock delay has triggered.
    fn reset_lock_delay(&mut self) {
        if self.lock_delay {
            self.drop_tick = 0;
        }
    }

    /// Drop the piece down one cell.
    ///
    /// If the piece can't drop, lock delay is started. If lock delay is over, the piece is locked.
    ///
    /// Returns whether this is a game over.
    fn drop_piece(&mut self) -> StepResult {
        self.piece.down();

        if self.piece_overlaps_board() {
            self.piece.up();
            if self.lock_delay {
                return self.lock_piece();
            } else {
                self.lock_delay = true;
            }
        } else if self.lock_delay {
            self.lock_delay = false;
        }

        StepResult::Continue
    }

    /// Lock the piece, clearing any rows and get a new piece from the bag.
    ///
    /// Returns whether this results in a game over.
    fn lock_piece(&mut self) -> StepResult {
        let FillResult {
            step_result,
            lines_cleared,
        } = self.board.lock_piece(&self.piece);

        self.piece = Piece::new(self.bag.pop());

        self.drop = Drop::Normal;
        self.drop_tick = 0;
        self.lock_delay = false;
        self.lines_cleared += lines_cleared;
        self.score += lines_cleared * lines_cleared * 100;

        if self.piece_overlaps_board() {
            StepResult::GameOver
        } else {
            step_result
        }
    }

    /// Return whether the piece is overlapping the board.
    fn piece_overlaps_board(&self) -> bool {
        let mut overlaps = false;

        for block in self.piece.blocks() {
            if !self.board.is_pos_free(block) {
                overlaps = true;
            }
        }

        overlaps
    }

    /// Perform a naive wall-kick (not SRS):
    /// 1. Try one space to the right
    /// 2. Try one space to the left
    fn try_wall_kick(&mut self) -> bool {
        self.try_move_right() || self.try_move_left()
    }
}

/// Result from applying a move or step in a game. The game may continue or it is a game over.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StepResult {
    Continue,
    GameOver,
}

/// A history of a game, that can be replayed. This is useful for verifying high scores.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct History {
    /// The seed used to initialise the game, so the game can be reliably replayed.
    seed: [u32; 4],

    /// A list of actions and when they occurred.
    actions: Vec<(Tick, Action)>,
}

impl History {
    /// Create a new empty history with the given seed.
    fn new(seed: [u32; 4]) -> Self {
        History {
            seed,
            actions: Vec::new(),
        }
    }

    /// Push an action onto the history, with the time it happened.
    ///
    /// Actions are assumed to be pushed chronologically.
    fn push_action(&mut self, tick: Tick, action: Action) {
        self.actions.push((tick, action));
    }

    /// Replay a game and return the resulting score.
    pub fn replay(&self) -> u32 {
        let mut game = GameState::new(self.seed);

        for &(action_tick, action) in &self.actions {
            while game.tick < action_tick {
                if game.apply_step() == StepResult::GameOver {
                    return game.score;
                }
            }

            game.apply_action(action);
        }

        // after actions stopped, the game will have continued until a game over
        loop {
            if game.apply_step() == StepResult::GameOver {
                return game.score;
            }
        }
    }
}
