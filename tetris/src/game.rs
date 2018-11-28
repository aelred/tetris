use board::Board;
use board::FillResult;
use game_over::GameOver;
use piece::Piece;
use shape::Bag;
use state::Paused;
use state::State;
use std::cmp;

use rand;
use rand::SeedableRng;
use rand::XorShiftRng;
use std::mem;
use std::ops::Add;
use std::ops::Mul;

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

pub struct GameWithHistory {
    pub game: Box<Game>,
    history: History,
}

impl Default for GameWithHistory {
    fn default() -> Self {
        let seed = rand::random();
        GameWithHistory {
            game: Box::new(Game::new(seed)),
            history: History::new(seed),
        }
    }
}

impl GameWithHistory {
    pub fn update(mut self) -> State {
        let is_game_over = self.game.apply_step();

        if is_game_over {
            let game_over = GameOver::new(self.game.score, self.history.clone());
            State::GameOver(game_over)
        } else {
            State::Play(self)
        }
    }

    pub fn move_left(&mut self) {
        self.apply_action(Action::MoveLeft);
    }

    pub fn move_right(&mut self) {
        self.apply_action(Action::MoveRight);
    }

    pub fn rotate(&mut self) {
        self.apply_action(Action::Rotate);
    }

    pub fn start_soft_drop(&mut self) {
        self.apply_action(Action::StartSoftDrop);
    }

    pub fn start_hard_drop(&mut self) {
        self.apply_action(Action::StartHardDrop);
    }

    pub fn stop_drop(&mut self) {
        self.apply_action(Action::StopDrop);
    }

    pub fn pause(self) -> State {
        State::Paused(Paused(self))
    }

    fn apply_action(&mut self, action: Action) {
        self.history.push_action(self.game.tick, action);
        self.game.apply_action(action);
    }
}

pub struct Game {
    pub piece: Piece,
    pub board: Board,
    pub bag: Bag,
    drop_tick: u32,
    lock_delay: bool,
    drop: Drop,
    pub lines_cleared: u32,
    pub score: u32,
    tick: Tick,
}

impl Game {
    fn new(seed: [u32; 4]) -> Game {
        let mut bag = Bag::new(XorShiftRng::from_seed(seed));
        Game {
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

    fn apply_step(&mut self) -> bool {
        self.tick.incr();

        while self.drop_tick >= Gravity::UNITS_PER_CELL {
            self.drop_tick -= Gravity::UNITS_PER_CELL;
            let is_game_over = self.drop_piece();
            if is_game_over {
                return true;
            }
        }

        self.drop_tick += match self.drop {
            Drop::Normal => self.normal_gravity(),
            Drop::Soft => Gravity::SOFT_DROP,
            Drop::Hard => Gravity::HARD_DROP,
        }.0;

        false
    }

    fn normal_gravity(&self) -> Gravity {
        let level = self.lines_cleared / NUM_LINES_CLEARED_PER_LEVEL;

        let g = Gravity::INITIAL + Gravity::INCREASE_PER_LEVEL * level;
        cmp::min(g, Gravity::SOFT_DROP)
    }

    fn try_rotate(&mut self) -> bool {
        self.piece.rotate_clockwise();
        self.reset_lock_delay();

        let successful_rotation = !self.piece_overlaps_board() || self.try_wall_kick();

        if !successful_rotation {
            self.piece.rotate_anticlockwise();
        }

        successful_rotation
    }

    fn try_move_left(&mut self) -> bool {
        self.piece.left();
        self.reset_lock_delay();

        let collides = self.piece_overlaps_board();

        if collides {
            self.piece.right();
        }

        !collides
    }

    fn try_move_right(&mut self) -> bool {
        self.piece.right();
        self.reset_lock_delay();

        let collides = self.piece_overlaps_board();

        if collides {
            self.piece.left();
        }

        !collides
    }

    fn reset_lock_delay(&mut self) {
        if self.lock_delay {
            self.drop_tick = 0;
        }
    }

    fn drop_piece(&mut self) -> bool {
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

        false
    }

    fn lock_piece(&mut self) -> bool {
        let new_piece = Piece::new(self.bag.pop());
        let old_piece = mem::replace(&mut self.piece, new_piece);

        let FillResult {
            mut is_game_over,
            lines_cleared,
        } = self.board.lock_piece(old_piece);

        self.drop = Drop::Normal;
        self.drop_tick = 0;
        self.lock_delay = false;
        self.lines_cleared += lines_cleared;
        self.score += lines_cleared * lines_cleared * 100;

        if self.piece_overlaps_board() {
            is_game_over = true;
        }

        is_game_over
    }

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct History {
    seed: [u32; 4],
    actions: Vec<(Tick, Action)>,
}

impl History {
    fn new(seed: [u32; 4]) -> Self {
        History {
            seed,
            actions: Vec::new(),
        }
    }

    fn push_action(&mut self, tick: Tick, action: Action) {
        self.actions.push((tick, action));
    }

    pub fn replay(&self) -> u32 {
        let mut game = Game::new(self.seed);

        for &(action_tick, action) in &self.actions {
            while game.tick < action_tick {
                let is_game_over = game.apply_step();
                if is_game_over {
                    return game.score;
                }
            }

            game.apply_action(action);
        }

        // after actions stopped, the game will have continued until a game over
        loop {
            let is_game_over = game.apply_step();
            if is_game_over {
                return game.score;
            }
        }
    }
}
