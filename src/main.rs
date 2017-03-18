#[macro_use]
mod macros;

mod tetromino;
mod pos;
mod board;
mod tile;
mod piece;
mod state;

extern crate sdl2;
extern crate rand;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use state::State;
use state::WINDOW_WIDTH;
use state::WINDOW_HEIGHT;

use std::thread::sleep;

use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::pixels::Color::RGB;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const TICK: u64 = 33;

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
