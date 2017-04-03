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

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::WindowEvent::FocusLost;

const INITIAL_GRAVITY: f32 = 0.04;
const LEVELS_BETWEEN_GRAVITY_INCREASE: u32 = 10;
const GRAVITY_INCREASE: f32 = 0.02;
const SOFT_DROP_GRAVITY: f32 = 1.0;
const HARD_DROP_GRAVITY: f32 = 20.0;

enum Gravity {
    Normal,
    SoftDrop,
    HardDrop,
}

pub struct Game {
    piece: Piece,
    board: Board,
    bag: Bag,
    drop_tick: f32,
    lock_delay: bool,
    gravity: Gravity,
    lines_cleared: u32,
    score: u32,
}

impl Game {
    pub fn new() -> Game {
        let mut bag = Bag::new();
        Game {
            piece: Piece::new(bag.pop()),
            board: Board::new(),
            bag: bag,
            drop_tick: 0.0,
            lock_delay: false,
            gravity: Gravity::Normal,
            lines_cleared: 0,
            score: 0,
        }
    }

    pub fn update(&mut self, drawer: &mut Drawer, events: &[Event]) -> StateChange {

        for event in events {
            match *event {
                Event::Window { win_event, .. } => {
                    if let FocusLost = win_event {
                        return StateChange::Push(State::Paused);
                    }
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left => self.left(),
                        Keycode::Right => self.right(),
                        Keycode::Up => self.rotate(),
                        Keycode::Down => self.gravity = Gravity::SoftDrop,
                        Keycode::Space => self.gravity = Gravity::HardDrop,
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Keycode::Down = keycode {
                        self.gravity = Gravity::Normal;
                    }
                }
                _ => {}
            }
        }

        drawer.set_viewport(*BOARD_BORDER_VIEW);
        self.board.draw_border(drawer);

        drawer.set_viewport(*BOARD_VIEW);
        self.board.draw(drawer);
        self.piece.draw(drawer);

        self.draw_next(drawer);

        self.draw_score(drawer);

        let is_game_over = self.update_piece();

        if is_game_over {
            StateChange::Replace(State::GameOver(GameOver::new(self.score)))
        } else {
            StateChange::None
        }
    }

    fn normal_gravity(&self) -> f32 {
        let level = self.lines_cleared / LEVELS_BETWEEN_GRAVITY_INCREASE;

        let g = INITIAL_GRAVITY + GRAVITY_INCREASE * level as f32;
        if g < SOFT_DROP_GRAVITY {
            g
        } else {
            SOFT_DROP_GRAVITY
        }
    }

    fn rotate(&mut self) {
        self.piece.rotate_clockwise();
        self.reset_lock_delay();

        if self.collides() {
            self.piece.rotate_anticlockwise();
        }
    }

    fn left(&mut self) {
        self.piece.left();
        self.reset_lock_delay();

        if self.collides() {
            self.piece.right();
        }
    }

    fn right(&mut self) {
        self.piece.right();
        self.reset_lock_delay();

        if self.collides() {
            self.piece.left();
        }
    }

    fn update_piece(&mut self) -> bool {
        while self.drop_tick >= 1.0 {
            self.drop_tick -= 1.0;
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

    fn reset_lock_delay(&mut self) {
        if self.lock_delay {
            self.drop_tick = 0.0;
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
        let FillResult { mut is_game_over, lines_cleared } =
            self.board.fill(self.piece.blocks(), self.piece.tetromino.color);

        self.piece = Piece::new(self.bag.pop());
        self.gravity = Gravity::Normal;
        self.drop_tick = 0.0;
        self.lock_delay = false;
        self.lines_cleared += lines_cleared;
        self.score += lines_cleared * lines_cleared * 100;

        if self.collides() {
            is_game_over = true;
        }

        is_game_over
    }

    fn draw_score(&self, drawer: &mut Drawer) {
        drawer.set_viewport(*SCORE_VIEW);

        drawer.text()
            .draw("lines")
            .size(2)
            .left()
            .draw(&self.lines_cleared.to_string())
            .size(1)
            .left()
            .offset(0, PAD as i32)
            .draw("score")
            .size(2)
            .left()
            .draw(&self.score.to_string());
    }

    fn draw_next(&self, drawer: &mut Drawer) {
        drawer.set_viewport(*PREVIEW_VIEW);

        drawer.draw_border(Pos::new(tetromino::WIDTH as i16, tetromino::HEIGHT as i16));
        self.bag.peek().draw(drawer, Rotation::default(), Pos::new(1, 1));
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

    static ref SCORE_VIEW: Rect = Rect::new(PREVIEW_X + BOARD_BORDER as i32 + PAD as i32, PAD as i32, PREVIEW_WIDTH, BOARD_HEIGHT);
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

const PAD: u32 = BLOCK_SIZE as u32;

pub const WINDOW_WIDTH: u32 = BOARD_WIDTH + BOARD_BORDER + PREVIEW_WIDTH;
pub const WINDOW_HEIGHT: u32 = TOTAL_BOARD_HEIGHT;
