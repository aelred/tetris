#[macro_use]
mod macros;

mod tetromino;
mod pos;
mod board;
mod tile;
mod piece;

extern crate sdl2;
extern crate rand;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use tile::TILE_SIZE;
use board::HIDE_ROWS;
use board::Board;
use board::WIDTH;
use board::HEIGHT;
use piece::Piece;

use std::thread::sleep;

use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color::RGB;
use sdl2::event::Event;
use sdl2::event::WindowEvent::{FocusGained, FocusLost};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const BOARD_BORDER: u32 = TILE_SIZE as u32;
const BOARD_WIDTH: u32 = WIDTH as u32 * TILE_SIZE as u32;
const BOARD_HEIGHT: u32 = (HEIGHT as u32 - HIDE_ROWS as u32) * TILE_SIZE as u32;

const PREVIEW_X: i32 = BOARD_WIDTH as i32 + BOARD_BORDER as i32;
const PREVIEW_Y: i32 = WINDOW_HEIGHT as i32 - (tetromino::HEIGHT + 2) as i32 * TILE_SIZE as i32;
const PREVIEW_WIDTH: u32 = (tetromino::WIDTH + 2) as u32 * TILE_SIZE as u32;
const PREVIEW_HEIGHT: u32 = (tetromino::HEIGHT + 2) as u32 * TILE_SIZE as u32;

const WINDOW_WIDTH: u32 = BOARD_WIDTH + BOARD_BORDER + PREVIEW_WIDTH;
const WINDOW_HEIGHT: u32 = BOARD_HEIGHT + BOARD_BORDER * 2;

const TICK: u64 = 33;

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

enum State {
    Play {
        piece: Box<Piece>,
        board: Box<Board>,
    },
    Paused,
}


impl State {
    fn play() -> State {
        State::Play {
            piece: Box::new(Piece::new()),
            board: Box::new(Board::new()),
        }
    }

    fn update(&mut self, renderer: &mut Renderer, events: &[Event]) -> StateChange {
        match *self {
            State::Play { ref mut piece, ref mut board } => {
                State::play_update(piece, board, renderer, events)
            }
            State::Paused => State::pause_update(events),
        }
    }

    fn play_update(piece: &mut Piece,
                   board: &mut Board,
                   renderer: &mut Renderer,
                   events: &[Event])
                   -> StateChange {
        renderer.set_viewport(Some(board_border_view()));
        board.draw_border(renderer);

        renderer.set_viewport(Some(board_view()));
        board.draw(renderer);
        piece.draw(renderer);

        renderer.set_viewport(Some(preview_view()));
        piece.draw_next(renderer);

        piece.update(board);

        let mut state_change = StateChange::None;

        for event in events {
            match *event {
                Event::Window { win_event, .. } => {
                    if let FocusLost = win_event {
                        state_change = StateChange::Push(State::Paused);
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

        state_change
    }

    fn pause_update(events: &[Event]) -> StateChange {
        for event in events {
            if let Event::Window { win_event: FocusGained, .. } = *event {
                return StateChange::Pop;
            }
        }

        StateChange::None
    }
}

enum StateChange {
    None,
    Push(State),
    Pop,
}

impl StateChange {
    fn apply(self, states: &mut Vec<State>) {
        match self {
            StateChange::None => {}
            StateChange::Push(state) => {
                states.push(state);
            }
            StateChange::Pop => {
                states.pop();
            }
        }
    }
}

fn main() {

    let sdl_context = sdl2::init().unwrap();

    let window = create_window(&sdl_context);

    let mut renderer = window.renderer().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    play_tetris(&mut renderer, &mut event_pump);
}

fn play_tetris(renderer: &mut Renderer, event_pump: &mut EventPump) {

    let mut states = Vec::new();
    let mut events = Vec::new();
    states.push(State::play());

    loop {
        renderer.set_draw_color(RGB(32, 48, 32));
        renderer.clear();

        events.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
                _ => {}
            }

            events.push(event);
        }

        let state_change = {
            let mut state = states.last_mut().unwrap();
            state.update(renderer, &events)
        };

        state_change.apply(&mut states);

        renderer.present();

        sleep(Duration::from_millis(TICK));
    }
}

fn create_window(sdl_context: &Sdl) -> Window {
    let video_subsystem = sdl_context.video().unwrap();

    video_subsystem.window("Tetris", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap()
}
