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
use board::Board;
use board::WIDTH;
use board::HEIGHT;
use piece::Piece;

use std::thread::sleep;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::pixels::Color::RGB;
use sdl2::event::Event::Quit;
use sdl2::event::Event::KeyDown;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WINDOW_WIDTH: u32 = WIDTH as u32 * TILE_SIZE as u32;
const WINDOW_HEIGHT: u32 = HEIGHT as u32 * TILE_SIZE as u32;
const TICK: u64 = 33;

fn main() {

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
                Quit { .. } => break 'main,
                KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'main,
                        Keycode::Left => piece.left(&board),
                        Keycode::Right => piece.right(&board),
                        Keycode::Up => piece.rotate(&board),
                        Keycode::Down => piece.drop(&mut board),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        board.draw(&renderer);

        piece.draw(&renderer);

        piece.update(&mut board);

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
