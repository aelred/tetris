use board::Board;
use tetromino;
use tetromino::Tetromino;
use tetromino::Bag;
use tetromino::Rotation;
use tetromino::ZERO_ROTATION;
use pos::Pos;
use board::WIDTH;
use tile::draw_border;

use sdl2::render::Renderer;

const NORMAL_GRAVITY: f32 = 0.1;
const SOFT_DROP_GRAVITY: f32 = 1.0;
const HARD_DROP_GRAVITY: f32 = 20.0;

const INITIAL_X: i8 = WIDTH as i8 / 2 - 2;

#[derive(Debug)]
pub struct Piece {
    tetromino: &'static Tetromino,
    next_tetromino: &'static Tetromino,
    rot: Rotation,
    pos: Pos,
    drop_tick: f32,
    lock_delay: bool,
    gravity: f32,
    bag: Bag,
}

impl Piece {
    pub fn new() -> Piece {
        let mut bag = Bag::new();

        Piece {
            tetromino: bag.next_tetromino(),
            next_tetromino: bag.next_tetromino(),
            rot: ZERO_ROTATION,
            pos: initial_pos(),
            drop_tick: 0.0,
            lock_delay: false,
            gravity: NORMAL_GRAVITY,
            bag: bag,
        }
    }

    pub fn update(&mut self, board: &mut Board) {
        while self.drop_tick >= 1.0 {
            self.drop_tick -= 1.0;
            self.drop(board);
        }

        self.drop_tick += self.gravity;
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
        self.gravity = SOFT_DROP_GRAVITY;
    }

    pub fn start_hard_drop(&mut self) {
        self.gravity = HARD_DROP_GRAVITY;
    }

    pub fn stop_drop(&mut self) {
        self.gravity = NORMAL_GRAVITY;
    }

    pub fn draw(&self, offset: Pos, next_pos: Pos, renderer: &Renderer) {
        self.tetromino.draw(self.rot, self.pos + offset, renderer);
        self.next_tetromino.draw(ZERO_ROTATION, next_pos, renderer);
        draw_border(renderer,
                    next_pos,
                    next_pos + Pos::new(tetromino::WIDTH as i8, tetromino::HEIGHT as i8));
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

        self.tetromino = self.next_tetromino;
        self.next_tetromino = self.bag.next_tetromino();
        self.rot = ZERO_ROTATION;
        self.pos = initial_pos();
        self.drop_tick = 0.0;
        self.gravity = NORMAL_GRAVITY;
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
            self.drop_tick = 0.0;
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
