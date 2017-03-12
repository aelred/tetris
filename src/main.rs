mod tetromino;
mod pos;

extern crate sdl2;
extern crate rand;

use pos::Pos;
use tetromino::Tetromino;
use tetromino::NUM_ROTATIONS;

use std::thread::sleep;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::pixels::Color::RGB;
use sdl2::event::Event::Quit;
use sdl2::event::Event::KeyDown;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::gfx::primitives::DrawRenderer;

const WIDTH: usize = 10;
const HEIGHT: usize = 24;
const TILE_SIZE: i16 = 24;

const WINDOW_WIDTH: u32 = WIDTH as u32 * TILE_SIZE as u32;
const WINDOW_HEIGHT: u32 = HEIGHT as u32 * TILE_SIZE as u32;
const TICK: u64 = 33;
const DROP_SPEED: u8 = 10;

const INITIAL_POS: Pos = Pos {
    x: WIDTH as isize / 2 - 2,
    y: -4,
};

struct Board([[Option<Color>; WIDTH]; HEIGHT]);

impl Board {
    fn new() -> Board {
        Board([[None; WIDTH]; HEIGHT])
    }

    fn touches(&self, pos: Pos) -> bool {
        pos.y >= 0 &&
        (pos.x < 0 || pos.x >= WIDTH as isize || pos.y >= HEIGHT as isize ||
         self.0[pos.y as usize][pos.x as usize].is_some())
    }

    fn fill(&mut self, pos: Pos, color: Color) {
        self.0[pos.y as usize][pos.x as usize] = Some(color);
    }

    fn check_clear(&mut self) {
        for y in 0..HEIGHT {
            let mut clear = true;

            'check_clear: for cell in self.0[y].iter() {
                if cell.is_none() {
                    clear = false;
                    break 'check_clear;
                }
            }

            if clear {
                for yy in (1..y + 1).rev() {
                    self.0[yy] = self.0[yy - 1];
                }

                for x in 0..WIDTH {
                    self.0[0][x] = None;
                }
            }
        }
    }

    fn draw(&self, renderer: &Renderer) {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match *cell {
                    Some(color) => {
                        draw_box(&renderer,
                                 Pos {
                                     x: x as isize,
                                     y: y as isize,
                                 },
                                 color)
                    }
                    None => (),
                }
            }
        }
    }
}

struct Piece {
    tetromino: &'static Tetromino,
    rot: usize,
    pos: Pos,
    drop_tick: u8,
    lock_delay: bool,
}

impl Piece {
    fn new() -> Piece {
        Piece {
            tetromino: Tetromino::random(),
            rot: 0,
            pos: INITIAL_POS,
            drop_tick: 0,
            lock_delay: false,
        }
    }

    fn update(&mut self, board: &mut Board) {
        if self.drop_tick == DROP_SPEED {
            self.drop(board);
            self.drop_tick = 0;
        }

        self.drop_tick += 1;
    }

    fn drop(&mut self, board: &mut Board) {
        self.pos.y += 1;

        if self.collides(board) {
            self.pos.y -= 1;
            if self.lock_delay {
                self.lock(board);
            } else {
                self.lock_delay = true;
            }
        } else if self.lock_delay {
            self.lock_delay = false;
        }
    }

    fn lock(&mut self, board: &mut Board) {
        self.tetromino.each_cell(self.rot,
                                 |pos| board.fill(self.pos + pos, self.tetromino.color));
        board.check_clear();

        self.tetromino = Tetromino::random();
        self.rot = 0;
        self.pos = INITIAL_POS;
        self.drop_tick = 0;
        self.lock_delay = false;
    }

    fn collides(&self, board: &Board) -> bool {
        let mut collides = false;

        self.tetromino.each_cell(self.rot, |pos| if board.touches(self.pos + pos) {
            collides = true;
        });

        collides
    }

    fn rotate(&mut self, board: &Board) {
        self.reset_lock_delay();

        let old_rot = self.rot;
        self.rot = (self.rot + 1) % NUM_ROTATIONS;

        if self.collides(board) {
            self.rot = old_rot;
        }
    }

    fn left(&mut self, board: &Board) {
        self.reset_lock_delay();

        self.pos.x -= 1;

        if self.collides(board) {
            self.pos.x += 1;
        }
    }

    fn right(&mut self, board: &Board) {
        self.reset_lock_delay();

        self.pos.x += 1;

        if self.collides(board) {
            self.pos.x -= 1;
        }
    }

    fn reset_lock_delay(&mut self) {
        if self.lock_delay {
            self.drop_tick = 0;
        }
    }

    fn draw(&self, renderer: &Renderer) {
        self.tetromino.each_cell(self.rot,
                                 |pos| draw_box(&renderer, self.pos + pos, self.tetromino.color));
    }
}

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

fn draw_box(renderer: &Renderer, pos: Pos, col: Color) {
    let x = pos.x as i16;
    let y = pos.y as i16;
    let _ = renderer.box_(x * TILE_SIZE + 1,
                          y * TILE_SIZE + 1,
                          (x + 1) * TILE_SIZE - 1,
                          (y + 1) * TILE_SIZE - 1,
                          col);
}
