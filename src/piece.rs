use tile::draw_tile;
use board::Board;
use tetromino::Tetromino;
use tetromino::Bag;
use tetromino::Rotation;
use pos::Pos;
use board::WIDTH;

use sdl2::render::Renderer;

const SOFT_DROP_SPEED: u8 = 10;
const GRAVITY: f32 = 0.1;
const RECIP_GRAVITY: u8 = (1.0 / GRAVITY) as u8;

const INITIAL_X: isize = WIDTH as isize / 2 - 2;

pub struct Piece {
    tetromino: &'static Tetromino,
    rot: Rotation,
    pos: Pos,
    drop_tick: u8,
    lock_delay: bool,
    soft_drop: bool,
    bag: Bag,
}

impl Piece {
    pub fn new() -> Piece {
        let mut bag = Bag::new();

        Piece {
            tetromino: bag.next(),
            rot: Rotation::new(),
            pos: initial_pos(),
            drop_tick: 0,
            lock_delay: false,
            soft_drop: false,
            bag: bag,
        }
    }

    pub fn update(&mut self, board: &mut Board) {
        while self.drop_tick >= RECIP_GRAVITY {
            self.drop_tick -= RECIP_GRAVITY;
            self.drop(board);
        }

        self.drop_tick += if self.soft_drop { SOFT_DROP_SPEED } else { 1 };
    }

    pub fn rotate(&mut self, board: &Board) {
        self.reset_lock_delay();

        let old_rot = self.rot;
        self.rot = self.rot.rotate();

        if self.collides(board) {
            self.rot = old_rot;
        }
    }

    pub fn left(&mut self, board: &Board) {
        self.reset_lock_delay();

        self.pos = self.pos.left();

        if self.collides(board) {
            self.pos = self.pos.right();
        }
    }

    pub fn right(&mut self, board: &Board) {
        self.reset_lock_delay();

        self.pos = self.pos.right();

        if self.collides(board) {
            self.pos = self.pos.left();
        }
    }

    pub fn start_soft_drop(&mut self) {
        self.soft_drop = true;
    }

    pub fn stop_soft_drop(&mut self) {
        self.soft_drop = false;
    }

    pub fn draw(&self, renderer: &Renderer) {
        self.each_cell(|pos| draw_tile(&renderer, pos, self.tetromino.color));
    }

    fn drop(&mut self, board: &mut Board) {
        self.pos = self.pos.down();

        if self.collides(board) {
            self.pos = self.pos.up();
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
        self.each_cell(|pos| board.fill(pos, self.tetromino.color));
        board.check_clear();

        self.tetromino = self.bag.next();
        self.rot = Rotation::new();
        self.pos = initial_pos();
        self.drop_tick = 0;
        self.lock_delay = false;
    }

    fn collides(&self, board: &Board) -> bool {
        let mut collides = false;

        self.each_cell(|pos| if board.touches(pos) {
                           collides = true;
                       });

        collides
    }

    fn reset_lock_delay(&mut self) {
        if self.lock_delay {
            self.drop_tick = 0;
        }
    }

    fn each_cell<F>(&self, mut f: F)
        where F: FnMut(Pos) -> ()
    {
        self.tetromino.each_cell(self.rot, |pos| f(self.pos + pos));
    }
}

fn initial_pos() -> Pos {
    Pos::new(INITIAL_X, 0)
}
