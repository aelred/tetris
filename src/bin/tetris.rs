extern crate tetris;

extern crate sdl2;

#[cfg(target_os = "emscripten")]
extern crate emscripten;
#[cfg(target_os = "emscripten")]
extern crate libc;

use tetris::state::State;
use tetris::draw::Drawer;

use std::cmp::max;

use sdl2::Sdl;
use sdl2::rwops::RWops;
use sdl2::ttf;
use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use tetris::draw::WINDOW_HEIGHT;
use tetris::draw::WINDOW_WIDTH;

const TICK: u64 = 33;

static FONT_DATA: &'static [u8] = include_bytes!("../../resources/8-BIT WONDER.TTF");

const FONT_MULTIPLE: u16 = 9;

// Funny division is done here to round to nearest multiple of FONT_MULTIPLE
const FONT_SIZE: u16 = (WINDOW_HEIGHT / 32) as u16 / FONT_MULTIPLE * FONT_MULTIPLE;

struct Context<'a> {
    drawer: Drawer<'a>,
    event_pump: EventPump,
    states: Vec<State>,
}

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let ttf_context = ttf::init().unwrap();

    let font_data = RWops::from_bytes(FONT_DATA).unwrap();
    let font_size = max(FONT_SIZE, FONT_MULTIPLE);
    let font = ttf_context
        .load_font_from_rwops(font_data, font_size)
        .unwrap();

    let window = create_window(&sdl_context);

    let mut context = Context {
        drawer: Drawer::new(window.renderer().build().unwrap(), font),
        event_pump: sdl_context.event_pump().unwrap(),
        states: Vec::new(),
    };

    context.states.push(State::Title);

    play_tetris(&mut context);
}

#[cfg(not(target_os = "emscripten"))]
fn play_tetris(context: &mut Context) {
    use std::thread::sleep;
    use std::time::Duration;

    loop {
        main_loop(context);
        sleep(Duration::from_millis(TICK));
    }
}

#[cfg(target_os = "emscripten")]
fn play_tetris(mut context: &mut Context) {
    use emscripten::em;
    use std::mem::transmute;

    extern "C" fn em_loop(arg: *mut libc::c_void) {
        let context = unsafe { transmute::<*mut libc::c_void, &mut Context>(arg) };
        main_loop(context);
    }

    em::set_main_loop_arg(
        em_loop,
        unsafe { transmute::<&mut Context, *mut libc::c_void>(&mut context) },
        (1000 / TICK) as i32,
        true,
    );
}

fn main_loop(context: &mut Context) {
    context.drawer.clear();

    let mut events = Vec::new();

    for event in context.event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => exit(),
            _ => {}
        }

        events.push(event);
    }

    let state_change = {
        let mut state = context.states.last_mut().unwrap();
        state.update(&mut context.drawer, &events)
    };

    state_change.apply(&mut context.states);

    context.drawer.present();
}

fn exit() {
    std::process::exit(0);
}

fn create_window(sdl_context: &Sdl) -> Window {
    let video_subsystem = sdl_context.video().unwrap();

    video_subsystem
        .window("Tetris", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap()
}
