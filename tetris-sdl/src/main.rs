use sdl2;

mod draw;
mod event;

use crate::draw::Drawer;

use std::cmp::max;

use crate::draw::WINDOW_HEIGHT;
use crate::draw::WINDOW_WIDTH;
use crate::event::EventHandler;
use sdl2::mixer::LoaderRWops;
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::rwops::RWops;
use sdl2::ttf;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;
use std::time::Instant;
use tetris::state::State;

const TIME_BETWEEN_UPDATES_IN_MS: u64 = 33;

static FONT_DATA: &[u8] = include_bytes!("../resources/8-BIT WONDER.TTF");
static MUSIC_DATA: &[u8] = include_bytes!("../resources/tetris.ogg");

const FONT_MULTIPLE: u16 = 9;

// Funny division is done here to round to nearest multiple of FONT_MULTIPLE
const FONT_SIZE: u16 = (WINDOW_HEIGHT / 32) as u16 / FONT_MULTIPLE * FONT_MULTIPLE;

struct Context<'a> {
    drawer: Drawer<'a>,
    event_handler: EventHandler,

    /// The game [State]. This is an [Option] so that update methods can
    /// [Option::take] the [State] and consume it.
    state: Option<State>,

    /// The last time the game was stepped forward
    last_update: Instant,
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

    let context = Context {
        drawer: Drawer::new(window.into_canvas().build().unwrap(), font),
        event_handler,
        state: Some(State::default()),
        last_update: Instant::now(),
    };

    play_tetris(context);
}

#[cfg(not(target_os = "emscripten"))]
fn play_tetris(mut context: Context<'_>) {
    use std::thread::sleep;
    use std::time::Duration;

    const TIME_BETWEEN_FRAMES_IN_MS: u64 = 33;

    let time_between_frames = Duration::from_millis(TIME_BETWEEN_FRAMES_IN_MS);

    loop {
        context.main_loop();
        sleep(time_between_frames);
    }
}

#[cfg(target_os = "emscripten")]
fn play_tetris(mut context: Context) {
    use libc;
    use std::mem::transmute;

    type EmArgCallbackFun = extern "C" fn(_: *mut libc::c_void);

    extern "C" {
        fn emscripten_set_main_loop_arg(
            func: EmArgCallbackFun,
            arg: *mut libc::c_void,
            fps: libc::c_int,
            simulate_infinite_loop: libc::c_int,
        );
    }

    fn set_main_loop_arg(
        func: EmArgCallbackFun,
        arg: *mut libc::c_void,
        fps: i32,
        simulate_infinite_loop: bool,
    ) {
        unsafe {
            emscripten_set_main_loop_arg(
                func,
                arg,
                fps,
                if simulate_infinite_loop { 1 } else { 0 },
            );
        }
    }

    extern "C" fn em_loop(arg: *mut libc::c_void) {
        let context = unsafe { transmute::<*mut libc::c_void, &mut Context>(arg) };
        context.main_loop();
    }

    context.drawer.present();

    // We box the context and pass that to `transmute`, instead of a reference. If we pass a
    // reference, the `context` will be dropped at the end of this method (even though there is
    // still a main loop running that needs it). A box will take ownership for us and then
    // be magically obliterated in the `transmute` method, without dropping the context.
    // This means we have deliberately introduced a memory leak! Yay!
    let boxed_context = Box::new(context);

    set_main_loop_arg(
        em_loop,
        unsafe { transmute::<Box<Context>, *mut libc::c_void>(boxed_context) },
        0,
        true,
    );
}

impl Context<'_> {
    fn main_loop(&mut self) {
        let mut state = self.state.take().unwrap();
        state = self.event_handler.handle(state);

        // Check if enough time has passed to tick the game forward.
        // This makes the game speed independent of the frame rate.
        let time_between_updates = Duration::from_millis(TIME_BETWEEN_UPDATES_IN_MS);
        let now = Instant::now();
        let time_since_last_update = now - self.last_update;
        let num_updates =
            time_since_last_update.subsec_millis() / time_between_updates.subsec_millis();

        for _ in 0..num_updates {
            state = state.update();
            self.last_update = now;
        }

        self.drawer.clear();
        self.drawer.draw_state(&state);
        self.drawer.present();

        self.state = Some(state);
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
