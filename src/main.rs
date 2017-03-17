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

fn main() {

    let board_border_view = Rect::new(0,
                                      0,
                                      BOARD_WIDTH + BOARD_BORDER * 2,
                                      BOARD_HEIGHT + BOARD_BORDER * 2);

    let board_view = Rect::new(BOARD_BORDER as i32,
                               BOARD_BORDER as i32,
                               BOARD_WIDTH,
                               BOARD_HEIGHT);

    let preview_view = Rect::new(PREVIEW_X, PREVIEW_Y, PREVIEW_WIDTH, PREVIEW_HEIGHT);

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

        if !paused {
            renderer.set_viewport(Some(board_border_view));

            board.draw_border(&renderer);

            renderer.set_viewport(Some(board_view));

            board.draw(&renderer);

            piece.draw(&renderer);

            renderer.set_viewport(Some(preview_view));

            piece.draw_next(&renderer);

            piece.update(&mut board);
        }

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
