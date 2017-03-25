use block::BLOCK_SIZE;
use board;
use board::HIDE_ROWS;
use board::Board;
use piece::Piece;
use state::State;
use state::StateChange;
use pos::Pos;
use tetromino;
use tetromino::Rotation;
use tetromino::Bag;
use block::draw_border;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::event::WindowEvent::FocusLost;

const NORMAL_GRAVITY: f32 = 0.1;
const SOFT_DROP_GRAVITY: f32 = 1.0;
const HARD_DROP_GRAVITY: f32 = 20.0;

pub struct Game {
    piece: Piece,
    board: Board,
    bag: Bag,
    drop_tick: f32,
    lock_delay: bool,
    gravity: f32,
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
            gravity: NORMAL_GRAVITY,
        }
    }

    pub fn update(&mut self, renderer: &mut Renderer, events: &[Event]) -> StateChange {

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
                        Keycode::Down => self.gravity = SOFT_DROP_GRAVITY,
                        Keycode::Space => self.gravity = HARD_DROP_GRAVITY,
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Keycode::Down = keycode {
                        self.gravity = NORMAL_GRAVITY;
                    }
                }
                _ => {}
            }
        }

        renderer.set_viewport(Some(*BOARD_BORDER_VIEW));
        self.board.draw_border(renderer);

        renderer.set_viewport(Some(*BOARD_VIEW));
        self.board.draw(renderer);
        self.piece.draw(renderer);

        self.draw_next(renderer);

        let is_game_over = self.update_piece();

        if is_game_over {
            StateChange::Replace(State::GameOver)
        } else {
            StateChange::None
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

        self.drop_tick += self.gravity;

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
        let mut is_game_over = self.board.fill(self.piece.blocks(), self.piece.tetromino.color);

        self.piece = Piece::new(self.bag.pop());
        self.gravity = NORMAL_GRAVITY;
        self.drop_tick = 0.0;
        self.lock_delay = false;

        if self.collides() {
            is_game_over = true;
        }

        is_game_over
    }

    fn draw_next(&self, renderer: &mut Renderer) {
        renderer.set_viewport(Some(*PREVIEW_VIEW));

        draw_border(renderer,
                    Pos::new(tetromino::WIDTH as i16, tetromino::HEIGHT as i16));
        self.bag.peek().draw(renderer, Rotation::default(), Pos::new(1, 1));
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

pub const WINDOW_WIDTH: u32 = BOARD_WIDTH + BOARD_BORDER + PREVIEW_WIDTH;
pub const WINDOW_HEIGHT: u32 = TOTAL_BOARD_HEIGHT;