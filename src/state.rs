use block::BLOCK_SIZE;
use board::HIDE_ROWS;
use board::Board;
use board::WIDTH;
use board::HEIGHT;
use piece::Piece;
use tetromino;

use sdl2::render::Renderer;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::event::Event;
use sdl2::event::WindowEvent::{FocusGained, FocusLost};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const BOARD_BORDER: u32 = BLOCK_SIZE as u32;
const BOARD_WIDTH: u32 = WIDTH as u32 * BLOCK_SIZE as u32;
const BOARD_HEIGHT: u32 = (HEIGHT as u32 - HIDE_ROWS as u32) * BLOCK_SIZE as u32;

const PREVIEW_X: i32 = BOARD_WIDTH as i32 + BOARD_BORDER as i32;
const PREVIEW_Y: i32 = WINDOW_HEIGHT as i32 - (tetromino::HEIGHT + 2) as i32 * BLOCK_SIZE as i32;
const PREVIEW_WIDTH: u32 = (tetromino::WIDTH + 2) as u32 * BLOCK_SIZE as u32;
const PREVIEW_HEIGHT: u32 = (tetromino::HEIGHT + 2) as u32 * BLOCK_SIZE as u32;

pub const WINDOW_WIDTH: u32 = BOARD_WIDTH + BOARD_BORDER + PREVIEW_WIDTH;
pub const WINDOW_HEIGHT: u32 = BOARD_HEIGHT + BOARD_BORDER * 2;

pub enum State {
    Title,
    Play {
        piece: Box<Piece>,
        board: Box<Board>,
    },
    Paused,
    GameOver,
}

impl State {
    fn play() -> State {
        State::Play {
            piece: Box::new(Piece::new()),
            board: Box::new(Board::new()),
        }
    }

    pub fn update(&mut self,
                  renderer: &mut Renderer,
                  font: &Font,
                  events: &[Event])
                  -> StateChange {
        match *self {
            State::Title => State::title_update(renderer, font, events),
            State::Play { ref mut piece, ref mut board } => {
                State::play_update(piece, board, renderer, events)
            }
            State::Paused => State::pause_update(renderer, font, events),
            State::GameOver => State::game_over_update(renderer, font, events),
        }
    }

    fn title_update(renderer: &mut Renderer, font: &Font, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::KeyDown { keycode: Some(keycode), .. } = *event {
                if let Keycode::Return = keycode {
                    return StateChange::Push(State::play());
                }
            }
        }

        let tetris_target = draw_text("Tetris", 0, 0, 4, renderer, font);

        draw_text("[ Press Enter ]",
                  0,
                  tetris_target.height() as i32,
                  1,
                  renderer,
                  font);


        StateChange::None
    }

    fn play_update(piece: &mut Piece,
                   board: &mut Board,
                   renderer: &mut Renderer,
                   events: &[Event])
                   -> StateChange {

        for event in events {
            match *event {
                Event::Window { win_event, .. } => {
                    if let FocusLost = win_event {
                        return StateChange::Push(State::Paused);
                    }
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left => piece.left(board),
                        Keycode::Right => piece.right(board),
                        Keycode::Up => piece.rotate(board),
                        Keycode::Down => piece.start_soft_drop(),
                        Keycode::Space => piece.start_hard_drop(),
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Keycode::Down = keycode {
                        piece.stop_drop()
                    }
                }
                _ => {}
            }
        }

        renderer.set_viewport(Some(board_border_view()));
        board.draw_border(renderer);

        renderer.set_viewport(Some(board_view()));
        board.draw(renderer);
        piece.draw(renderer);

        renderer.set_viewport(Some(preview_view()));
        piece.draw_next(renderer);

        let is_game_over = piece.update(board);

        if is_game_over {
            StateChange::Replace(State::GameOver)
        } else {
            StateChange::None
        }
    }

    fn pause_update(renderer: &mut Renderer, font: &Font, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::Window { win_event: FocusGained, .. } = *event {
                return StateChange::Pop;
            }
        }

        renderer.set_viewport(None);

        draw_text("Paused", 0, 0, 1, renderer, font);

        StateChange::None
    }

    fn game_over_update(renderer: &mut Renderer, font: &Font, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::KeyDown { keycode: Some(keycode), .. } = *event {
                if let Keycode::Return = keycode {
                    return StateChange::Replace(State::play());
                }
            }
        }

        let game_over_target = draw_text("Game Over", 0, 0, 3, renderer, font);

        draw_text("[ Press Enter ]",
                  0,
                  game_over_target.height() as i32,
                  1,
                  renderer,
                  font);

        StateChange::None
    }
}

pub enum StateChange {
    None,
    Push(State),
    Pop,
    Replace(State),
}

impl StateChange {
    pub fn apply(self, states: &mut Vec<State>) {
        match self {
            StateChange::None => {}
            StateChange::Push(state) => {
                states.push(state);
            }
            StateChange::Pop => {
                states.pop();
            }
            StateChange::Replace(state) => {
                states.pop();
                states.push(state);
            }
        }
    }
}

fn draw_text(text: &str,
             offset_x: i32,
             offset_y: i32,
             size: u32,
             renderer: &mut Renderer,
             font: &Font)
             -> Rect {
    let surface = font.render(text).solid(Color::RGB(255, 255, 255)).unwrap();
    let texture = renderer.create_texture_from_surface(&surface).unwrap();

    let TextureQuery { width, height, .. } = texture.query();

    let target = center_view(offset_x, offset_y, width * size, height * size);

    renderer.copy(&texture, None, Some(target)).unwrap();

    target
}

fn board_border_view() -> Rect {
    Rect::new(0,
              0,
              BOARD_WIDTH + BOARD_BORDER * 2,
              BOARD_HEIGHT + BOARD_BORDER * 2)
}

fn board_view() -> Rect {
    Rect::new(BOARD_BORDER as i32,
              BOARD_BORDER as i32,
              BOARD_WIDTH,
              BOARD_HEIGHT)
}

fn preview_view() -> Rect {
    Rect::new(PREVIEW_X, PREVIEW_Y, PREVIEW_WIDTH, PREVIEW_HEIGHT)
}

fn center_view(x: i32, y: i32, width: u32, height: u32) -> Rect {
    let center_x = (WINDOW_WIDTH / 2) as i32;
    let center_y = (WINDOW_HEIGHT / 2) as i32;

    Rect::new(center_x + x - width as i32 / 2,
              center_y + y - height as i32 / 2,
              width,
              height)
}
