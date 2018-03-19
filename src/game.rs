use draw::BLOCK_SIZE;
use board;
use board::HIDE_ROWS;
use board::Board;
use board::FillResult;
use piece::Piece;
use state::State;
use state::StateChange;
use pos::Pos;
use tetromino;
use tetromino::Rotation;
use tetromino::Bag;
use draw::Drawer;
use game_over::GameOver;
use std::cmp;

use rand;
use rand::XorShiftRng;
use rand::SeedableRng;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::WindowEvent::FocusLost;

const INITIAL_GRAVITY: u32 = 4;
const GRAVITY_UNIT: u32 = 100;
const LEVELS_BETWEEN_GRAVITY_INCREASE: u32 = 10;
const GRAVITY_INCREASE: u32 = 2;
const SOFT_DROP_GRAVITY: u32 = GRAVITY_UNIT;
const HARD_DROP_GRAVITY: u32 = GRAVITY_UNIT * 20;

// the minimum velocity before movement is registered, in % of screen width per ms
const FINGER_SENSITIVITY: f32 = 0.0002;

enum Gravity {
    Normal,
    SoftDrop,
    HardDrop,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Rotate,
    StartSoftDrop,
    StartHardDrop,
    StopDrop,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialOrd, PartialEq, Debug)]
pub struct Tick(u32);

impl Tick {
    fn new() -> Tick {
        Tick(0)
    }

    fn incr(&mut self) {
        self.0 += 1;
    }
}

pub struct FingerPress {
    x: f32,
    y: f32,
    timestamp: u32,
}

impl FingerPress {
    fn velocity(self, other: &FingerPress) -> (f32, f32) {
        let dx = self.x - other.x;
        let dy = (self.y - other.y) * WINDOW_RATIO;
        let dt = (self.timestamp - other.timestamp) as f32;
        (dx / dt, dy / dt)
    }
}

pub struct GamePlay {
    game: Game,
    history: History,
    last_finger_press: Option<FingerPress>,
}

impl Default for GamePlay {
    fn default() -> Self {
        let seed = rand::random();
        GamePlay {
            game: Game::new(seed),
            history: History::new(seed),
            last_finger_press: None,
        }
    }
}

impl GamePlay {
    pub fn update(&mut self, drawer: &mut Drawer, events: &[Event]) -> StateChange {
        let mut actions = Vec::new();

        for event in events {
            match *event {
                Event::Window { win_event: FocusLost, .. } => {
                    return StateChange::Push(State::Paused);
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left => actions.push(Action::MoveLeft),
                        Keycode::Right => actions.push(Action::MoveRight),
                        Keycode::Up => actions.push(Action::Rotate),
                        Keycode::Down => actions.push(Action::StartSoftDrop),
                        Keycode::Space => actions.push(Action::StartHardDrop),
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    actions.push(Action::StopDrop);
                }
                Event::FingerDown { x, y, timestamp, .. } => {
                    self.last_finger_press = Some(FingerPress { x, y, timestamp });
                }
                Event::FingerUp { x, y, timestamp, .. } => {
                    if let Some(ref last_finger_press) = self.last_finger_press {
                        let finger_press = FingerPress { x, y, timestamp };
                        let (vx, vy) = finger_press.velocity(last_finger_press);
                        let action = if vx < -FINGER_SENSITIVITY {
                            Action::MoveLeft
                        } else if vx > FINGER_SENSITIVITY {
                            Action::MoveRight
                        } else if vy > FINGER_SENSITIVITY {
                            Action::StartHardDrop
                        } else {
                            Action::Rotate
                        };
                        actions.push(action);
                    }
                    self.last_finger_press = None;
                }
                _ => {}
            }
        }

        for action in actions {
            self.history.push(self.game.tick, action);
            self.game.apply_action(action);
        }

        let is_game_over = self.game.apply_step();

        drawer.set_viewport(*BOARD_BORDER_VIEW);
        self.game.board.draw_border(drawer);

        drawer.set_viewport(*BOARD_VIEW);
        self.game.board.draw(drawer);
        self.game.piece.draw(drawer);

        self.draw_next(drawer);

        self.draw_score(drawer);

        if is_game_over {
            let game_over = GameOver::new(self.game.score, self.history.clone());
            StateChange::Replace(State::GameOver(game_over))
        } else {
            StateChange::None
        }
    }

    fn draw_score(&self, drawer: &mut Drawer) {
        drawer.set_viewport(*SCORE_VIEW);

        drawer
            .text()
            .draw("lines")
            .size(2)
            .left()
            .draw(&self.game.lines_cleared.to_string())
            .size(1)
            .left()
            .offset(0, PAD)
            .draw("score")
            .size(2)
            .left()
            .draw(&self.game.score.to_string());
    }

    fn draw_next(&self, drawer: &mut Drawer) {
        drawer.set_viewport(*PREVIEW_VIEW);

        drawer.draw_border(Pos::new(tetromino::WIDTH as i16, tetromino::HEIGHT as i16));
        self.game.bag.peek().draw(
            drawer,
            Rotation::default(),
            Pos::new(1, 1),
        );
    }
}

pub struct Game {
    piece: Piece,
    board: Board,
    bag: Bag,
    drop_tick: u32,
    lock_delay: bool,
    gravity: Gravity,
    lines_cleared: u32,
    score: u32,
    tick: Tick,
}

impl Game {
    pub fn new(seed: [u32; 4]) -> Game {
        let mut bag = Bag::new(XorShiftRng::from_seed(seed));
        Game {
            piece: Piece::new(bag.pop()),
            board: Board::new(),
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
            Action::MoveLeft => {self.try_move_left();},
            Action::MoveRight => {self.try_move_right();},
            Action::Rotate => {self.try_rotate();},
            Action::StartSoftDrop => self.gravity = Gravity::SoftDrop,
            Action::StartHardDrop => self.gravity = Gravity::HardDrop,
            Action::StopDrop => self.gravity = Gravity::Normal,
        }
    }

    fn apply_step(&mut self) -> bool {
        self.tick.incr();

        while self.drop_tick >= GRAVITY_UNIT {
            self.drop_tick -= GRAVITY_UNIT;
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
        let FillResult {
            mut is_game_over,
            lines_cleared,
        } = self.board.fill(
            self.piece.blocks(),
            self.piece.tetromino.color,
        );

        self.piece = Piece::new(self.bag.pop());
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
            if self.board.touches(block) {
                collides = true;
            }
        }

        collides
    }

    /// Perform a naive wall-kick (not SRS):
    /// 1. Try one space to the right
    /// 2. Try one space to the left
    fn try_wall_kick(&mut self) -> bool {
        return self.try_move_right() || self.try_move_left()
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

lazy_static! {
    static ref BOARD_BORDER_VIEW: Rect = Rect::new(0,
                                                   0,
                                                   BOARD_WIDTH + BOARD_BORDER * 2,
                                                   BOARD_HEIGHT + BOARD_BORDER * 2);

    static ref BOARD_VIEW: Rect = Rect::new(BOARD_BORDER as i32,
                                            BOARD_BORDER as i32,
                                            BOARD_WIDTH,
                                            BOARD_HEIGHT);

    static ref PREVIEW_VIEW: Rect = Rect::new(PREVIEW_X, PREVIEW_Y, PREVIEW_WIDTH, PREVIEW_HEIGHT);

    static ref SCORE_VIEW: Rect = Rect::new(SCORE_X, PAD, PREVIEW_WIDTH, BOARD_HEIGHT);
}

const BOARD_BORDER: u32 = BLOCK_SIZE as u32;
const BOARD_WIDTH: u32 = board::WIDTH as u32 * BLOCK_SIZE as u32;
const BOARD_HEIGHT: u32 = (board::HEIGHT as u32 - HIDE_ROWS as u32) * BLOCK_SIZE as u32;
const TOTAL_BOARD_HEIGHT: u32 = BOARD_HEIGHT + BOARD_BORDER * 2;

const PREVIEW_X: i32 = BOARD_WIDTH as i32 + BOARD_BORDER as i32;
const PREVIEW_Y: i32 = TOTAL_BOARD_HEIGHT as i32 -
    (tetromino::HEIGHT + 2) as i32 * BLOCK_SIZE as i32;
const PREVIEW_WIDTH: u32 = (tetromino::WIDTH + 2) as u32 * BLOCK_SIZE as u32;
const PREVIEW_HEIGHT: u32 = (tetromino::HEIGHT + 2) as u32 * BLOCK_SIZE as u32;

const SCORE_X: i32 = PREVIEW_X + BOARD_BORDER as i32 + PAD;

const PAD: i32 = BLOCK_SIZE as i32;

pub const WINDOW_WIDTH: u32 = BOARD_WIDTH + BOARD_BORDER + PREVIEW_WIDTH;
pub const WINDOW_HEIGHT: u32 = TOTAL_BOARD_HEIGHT;
pub const WINDOW_RATIO: f32 = WINDOW_HEIGHT as f32 / WINDOW_WIDTH as f32;
