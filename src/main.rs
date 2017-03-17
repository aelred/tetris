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
use tile::HIDE_ROWS;
use board::Board;
use board::WIDTH;
use board::HEIGHT;
use piece::Piece;
use pos::Pos;

use std::thread::sleep;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::pixels::Color::RGB;
use sdl2::event::Event;
use sdl2::event::WindowEvent::{FocusGained, FocusLost};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const LEFT_BORDER: u8 = 1;
const TOP_BORDER: u8 = 1;
const RIGHT_BORDER: u8 = 6;
const BOTTOM_BORDER: u8 = 1;

const NEXT_PIECE_X: u8 = LEFT_BORDER + WIDTH + 1;
const NEXT_PIECE_Y: u8 = TOP_BORDER + HEIGHT - tetromino::HEIGHT;

const WINDOW_WIDTH: u8 = WIDTH + LEFT_BORDER + RIGHT_BORDER;
const WINDOW_HEIGHT: u8 = HEIGHT - HIDE_ROWS + TOP_BORDER + BOTTOM_BORDER;

const TICK: u64 = 33;

fn main() {

    let board_pos = Pos::new(LEFT_BORDER as i16, TOP_BORDER as i16);
    let next_pos = Pos::new(NEXT_PIECE_X as i16, NEXT_PIECE_Y as i16);

    let mut paused = false;

    let mut board = Board::new();
    let mut piece = Piece::new();

    let sdl_context = sdl2::init().unwrap();

    let window = create_window(&sdl_context);

    let mut renderer = window.renderer().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {

        renderer.set_draw_color(RGB(32, 48, 32));
        renderer.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window { win_event, .. } => {
                    match win_event {
                        FocusGained => paused = false,
                        FocusLost => paused = true,
                        _ => {}
                    }
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'main,
                        Keycode::Left => piece.left(&board),
                        Keycode::Right => piece.right(&board),
                        Keycode::Up => piece.rotate(&board),
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

        board.draw(board_pos, &renderer);

        piece.draw(board_pos, next_pos, &renderer);

        if !paused {
            piece.update(&mut board);
        }

        renderer.present();

        sleep(Duration::from_millis(TICK));
    }
}

fn create_window(sdl_context: &Sdl) -> Window {
    let video_subsystem = sdl_context.video().unwrap();

    video_subsystem.window("Tetris",
                           WINDOW_WIDTH as u32 * TILE_SIZE as u32,
                           WINDOW_HEIGHT as u32 * TILE_SIZE as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap()
}
