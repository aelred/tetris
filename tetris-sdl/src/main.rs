extern crate tetris;

extern crate sdl2;

#[cfg(target_os = "emscripten")]
extern crate libc;

#[macro_use]
extern crate lazy_static;

mod draw;
mod event;

use draw::Drawer;

use std::cmp::max;

use sdl2::Sdl;
use sdl2::rwops::RWops;
use sdl2::ttf;
use sdl2::video::Window;
use draw::WINDOW_HEIGHT;
use draw::WINDOW_WIDTH;
use event::EventHandler;
use sdl2::mixer::{DEFAULT_CHANNELS, AUDIO_S16LSB};
use sdl2::mixer::LoaderRWops;
use tetris::Tetris;

const TICK: u64 = 33;

static FONT_DATA: &'static [u8] = include_bytes!("../resources/8-BIT WONDER.TTF");
static MUSIC_DATA: &'static [u8] = include_bytes!("../resources/tetris.mid");

const FONT_MULTIPLE: u16 = 9;

// Funny division is done here to round to nearest multiple of FONT_MULTIPLE
const FONT_SIZE: u16 = (WINDOW_HEIGHT / 32) as u16 / FONT_MULTIPLE * FONT_MULTIPLE;

struct Context<'a> {
    drawer: Drawer<'a>,
    event_handler: EventHandler,
    tetris: Tetris,
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

    let frequency = 44_100;
    let format = AUDIO_S16LSB;
    let channels = DEFAULT_CHANNELS;
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

    let music_data = RWops::from_bytes(MUSIC_DATA).unwrap();
    let music = music_data.load_music().unwrap();
    music.play(1).unwrap();

    let event_handler = EventHandler::new(sdl_context.event_pump().unwrap());

    let mut context = Context {
        drawer: Drawer::new(window.into_canvas().build().unwrap(), font),
        event_handler,
        tetris: Tetris::default(),
    };

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
    use std::mem::transmute;

    type EmArgCallbackFun = extern fn(_: *mut libc::c_void);

    extern "C" {
        fn emscripten_set_main_loop_arg(func: EmArgCallbackFun, arg: *mut libc::c_void, fps: libc::c_int, simulate_infinite_loop: libc::c_int);
    }

    fn set_main_loop_arg(func: EmArgCallbackFun, arg: *mut libc::c_void, fps: i32, simulate_infinite_loop: bool) {
        unsafe {
            emscripten_set_main_loop_arg(func, arg, fps, if simulate_infinite_loop { 1 } else { 0 });
        }
    }

    extern "C" fn em_loop(arg: *mut libc::c_void) {
        let context = unsafe { transmute::<*mut libc::c_void, &mut Context>(arg) };
        context.main_loop();
    }

    set_main_loop_arg(
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
        let state_change = self.event_handler.handle(self.tetris.state());
        self.tetris.apply_state_change(state_change);
    }

    fn update_state(&mut self) {
        let state_change = self.tetris.state().update();
        self.tetris.apply_state_change(state_change);
    }

    fn draw(&mut self) {
        self.drawer.clear();
        self.drawer.draw_state(self.tetris.state());
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
