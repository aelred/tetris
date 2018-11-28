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

const INITIAL_GRAVITY: u32 = 4;
const GRAVITY_UNITS_PER_BLOCK: u32 = 100;
const LEVELS_BETWEEN_GRAVITY_INCREASE: u32 = 10;
const GRAVITY_INCREASE: u32 = 2;
const SOFT_DROP_GRAVITY: u32 = GRAVITY_UNITS_PER_BLOCK;
const HARD_DROP_GRAVITY: u32 = GRAVITY_UNITS_PER_BLOCK * 20;

enum Gravity {
    Normal,
    SoftDrop,
    HardDrop,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum Action {
    MoveLeft,
    MoveRight,
    Rotate,
    StartSoftDrop,
    StartHardDrop,
    StopDrop,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialOrd, PartialEq, Debug)]
struct Tick(u32);

impl Tick {
    fn new() -> Tick {
        Tick(0)
    }

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
        self.history.push(self.game.tick, action);
        self.game.apply_action(action);
    }
}

pub struct Game {
    pub piece: Piece,
    pub board: Board,
    pub bag: Bag,
    drop_tick: u32,
    lock_delay: bool,
    gravity: Gravity,
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
            gravity: Gravity::Normal,
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
            Action::StartSoftDrop => self.gravity = Gravity::SoftDrop,
            Action::StartHardDrop => self.gravity = Gravity::HardDrop,
            Action::StopDrop => self.gravity = Gravity::Normal,
        }
    }

    fn apply_step(&mut self) -> bool {
        self.tick.incr();

        while self.drop_tick >= GRAVITY_UNITS_PER_BLOCK {
            self.drop_tick -= GRAVITY_UNITS_PER_BLOCK;
            let is_game_over = self.drop();
            if is_game_over {
                return true;
            }
        }

        self.drop_tick += match self.gravity {
            Gravity::Normal => self.normal_gravity(),
            Gravity::SoftDrop => SOFT_DROP_GRAVITY,
            Gravity::HardDrop => HARD_DROP_GRAVITY,
        };

        false
    }

    fn normal_gravity(&self) -> u32 {
        let level = self.lines_cleared / LEVELS_BETWEEN_GRAVITY_INCREASE;

        let g = INITIAL_GRAVITY + GRAVITY_INCREASE * level;
        cmp::min(g, SOFT_DROP_GRAVITY)
    }

    fn try_rotate(&mut self) -> bool {
        self.piece.rotate_clockwise();
        self.reset_lock_delay();

        let successful_rotation = !self.collides() || self.try_wall_kick();

        if !successful_rotation {
            self.piece.rotate_anticlockwise();
        }

        successful_rotation
    }

    fn try_move_left(&mut self) -> bool {
        self.piece.left();
        self.reset_lock_delay();

        let collides = self.collides();

        if collides {
            self.piece.right();
        }

        !collides
    }

    fn try_move_right(&mut self) -> bool {
        self.piece.right();
        self.reset_lock_delay();

        let collides = self.collides();

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

    fn drop(&mut self) -> bool {
        self.piece.down();

        if self.collides() {
            self.piece.up();
            if self.lock_delay {
                return self.lock();
            } else {
                self.lock_delay = true;
            }
        } else if self.lock_delay {
            self.lock_delay = false;
        }

        false
    }

    fn lock(&mut self) -> bool {
        let new_piece = Piece::new(self.bag.pop());
        let old_piece = mem::replace(&mut self.piece, new_piece);

        let FillResult {
            mut is_game_over,
            lines_cleared,
        } = self.board.lock_piece(old_piece);

        self.gravity = Gravity::Normal;
        self.drop_tick = 0;
        self.lock_delay = false;
        self.lines_cleared += lines_cleared;
        self.score += lines_cleared * lines_cleared * 100;

        if self.collides() {
            is_game_over = true;
        }

        is_game_over
    }

    fn collides(&self) -> bool {
        let mut collides = false;

        for block in self.piece.blocks() {
            if !self.board.is_pos_free(block) {
                collides = true;
            }
        }

        collides
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

    fn push(&mut self, tick: Tick, action: Action) {
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
