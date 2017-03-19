use block::BLOCK_SIZE;
use board;
use board::HIDE_ROWS;
use board::Board;
use piece::Piece;
use state::State;
use state::StateChange;
use tetromino;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::event::WindowEvent::FocusLost;

pub struct Game {
    piece: Piece,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            piece: Piece::new(),
            board: Board::new(),
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
                        Keycode::Left => self.piece.left(&self.board),
                        Keycode::Right => self.piece.right(&self.board),
                        Keycode::Up => self.piece.rotate(&self.board),
                        Keycode::Down => self.piece.start_soft_drop(),
                        Keycode::Space => self.piece.start_hard_drop(),
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Keycode::Down = keycode {
                        self.piece.stop_drop()
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

        renderer.set_viewport(Some(*PREVIEW_VIEW));
        self.piece.draw_next(renderer);

        let is_game_over = self.piece.update(&mut self.board);

        if is_game_over {
            StateChange::Replace(State::GameOver)
        } else {
            StateChange::None
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
