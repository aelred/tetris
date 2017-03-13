use tile::draw_tile;
use board::Board;
use tetromino::Tetromino;
use tetromino::Bag;
use tetromino::Rotation;
use pos::Pos;
use board::WIDTH;

use sdl2::render::Renderer;

const GRAVITY: f32 = 0.1;
const RECIP_GRAVITY: u8 = (1.0 / GRAVITY) as u8;

const INITIAL_POS: Pos = Pos {
    x: WIDTH as isize / 2 - 2,
    y: -4,
};

pub struct Piece {
    tetromino: &'static Tetromino,
    rot: Rotation,
    pos: Pos,
    drop_tick: u8,
    lock_delay: bool,
    bag: Bag,
}

impl Piece {
    pub fn new() -> Piece {
        let mut bag = Bag::new();

        Piece {
            tetromino: bag.next(),
            rot: Rotation::new(),
            pos: INITIAL_POS,
            drop_tick: 0,
            lock_delay: false,
            bag: bag,
        }
    }

    pub fn update(&mut self, board: &mut Board) {
        if self.drop_tick == RECIP_GRAVITY {
            self.drop(board);
            self.drop_tick = 0;
        }

        self.drop_tick += 1;
    }

    pub fn drop(&mut self, board: &mut Board) {
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

        self.pos.x -= 1;

        if self.collides(board) {
            self.pos.x += 1;
        }
    }

    pub fn right(&mut self, board: &Board) {
        self.reset_lock_delay();

        self.pos.x += 1;

        if self.collides(board) {
            self.pos.x -= 1;
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        self.each_cell(|pos| draw_tile(&renderer, pos, self.tetromino.color));
    }

    fn lock(&mut self, board: &mut Board) {
        self.each_cell(|pos| board.fill(pos, self.tetromino.color));
        board.check_clear();

        self.tetromino = self.bag.next();
        self.rot = Rotation::new();
        self.pos = INITIAL_POS;
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
