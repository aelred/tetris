extern crate tetris;

extern crate sdl2;

#[cfg(target_os = "emscripten")]
extern crate emscripten;
#[cfg(target_os = "emscripten")]
extern crate libc;

#[macro_use]
extern crate lazy_static;

mod draw;
mod event;

use tetris::state::State;
use draw::Drawer;

use std::cmp::max;

use sdl2::Sdl;
use sdl2::rwops::RWops;
use sdl2::ttf;
use sdl2::video::Window;
use draw::WINDOW_HEIGHT;
use draw::WINDOW_WIDTH;
use event::EventHandler;

const TICK: u64 = 33;

static FONT_DATA: &'static [u8] = include_bytes!("../resources/8-BIT WONDER.TTF");

const FONT_MULTIPLE: u16 = 9;

// Funny division is done here to round to nearest multiple of FONT_MULTIPLE
const FONT_SIZE: u16 = (WINDOW_HEIGHT / 32) as u16 / FONT_MULTIPLE * FONT_MULTIPLE;

struct Context<'a> {
    drawer: Drawer<'a>,
    event_handler: EventHandler,
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

    let event_handler = EventHandler::new(sdl_context.event_pump().unwrap());

    let mut context = Context {
        drawer: Drawer::new(window.into_canvas().build().unwrap(), font),
        event_handler,
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
        context.main_loop();
        sleep(Duration::from_millis(TICK));
    }
}

#[cfg(target_os = "emscripten")]
fn play_tetris(mut context: &mut Context) {
    use emscripten::em;
    use std::mem::transmute;

    extern "C" fn em_loop(arg: *mut libc::c_void) {
        let context = unsafe { transmute::<*mut libc::c_void, &mut Context>(arg) };
        context.main_loop();
    }

    em::set_main_loop_arg(
        em_loop,
        unsafe { transmute::<&mut Context, *mut libc::c_void>(&mut context) },
        (1000 / TICK) as i32,
        true,
    );
}

impl <'a> Context<'a> {
    fn main_loop(&mut self) {
        self.handle_events();
        self.update_state();
        self.draw();
    }

    fn handle_events(&mut self) {
        let state_change = {
            let state = self.states.last_mut().unwrap();
            self.event_handler.handle(state)
        };
        state_change.apply(&mut self.states);
    }

    fn update_state(&mut self) {
        let state_change = {
            let state = self.states.last_mut().unwrap();
            state.update()
        };
        state_change.apply(&mut self.states);
    }

    fn draw(&mut self) {
        self.drawer.clear();
        let state = self.states.last_mut().unwrap();
        self.drawer.draw_state(state);
        self.drawer.present();
    }
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
