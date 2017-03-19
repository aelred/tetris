use board::Board;
use tetromino;
use tetromino::Tetromino;
use tetromino::Bag;
use tetromino::Rotation;
use pos::Pos;
use board::WIDTH;
use board::HIDE_ROWS;
use block::draw_border;

use sdl2::render::Renderer;

const NORMAL_GRAVITY: f32 = 0.1;
const SOFT_DROP_GRAVITY: f32 = 1.0;
const HARD_DROP_GRAVITY: f32 = 20.0;

const INITIAL_X: i16 = WIDTH as i16 / 2 - 2;

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
            rot: Rotation::default(),
            pos: initial_pos(),
            drop_tick: 0.0,
            lock_delay: false,
            gravity: NORMAL_GRAVITY,
            bag: bag,
        }
    }

    pub fn update(&mut self, board: &mut Board) -> bool {
        while self.drop_tick >= 1.0 {
            self.drop_tick -= 1.0;
            let is_game_over = self.drop(board);
            if is_game_over {
                return true;
            }
        }

        self.drop_tick += self.gravity;

        false
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

    pub fn draw(&self, renderer: &Renderer) {
        self.tetromino.draw(renderer,
                            self.rot,
                            self.pos + Pos::new(0, -(HIDE_ROWS as i16)));
    }

    pub fn draw_next(&self, renderer: &Renderer) {
        draw_border(renderer,
                    Pos::new(tetromino::WIDTH as i16, tetromino::HEIGHT as i16));
        self.next_tetromino.draw(renderer, Rotation::default(), Pos::new(1, 1));
    }

    fn drop(&mut self, board: &mut Board) -> bool {
        self.pos = self.pos.down();

        if self.collides(board) {
            self.pos = self.pos.up();
            if self.lock_delay {
                return self.lock(board);
            } else {
                self.lock_delay = true;
            }
        } else if self.lock_delay {
            self.lock_delay = false;
        }

        false
    }

    fn lock(&mut self, board: &mut Board) -> bool {
        let mut is_game_over = board.fill(self.blocks(), self.tetromino.color);

        self.tetromino = self.next_tetromino;
        self.next_tetromino = self.bag.next_tetromino();
        self.rot = Rotation::default();
        self.pos = initial_pos();
        self.drop_tick = 0.0;
        self.gravity = NORMAL_GRAVITY;
        self.lock_delay = false;

        if self.collides(board) {
            is_game_over = true;
        }

        is_game_over
    }

    fn collides(&self, board: &Board) -> bool {
        let mut collides = false;

        for block in self.blocks() {
            if board.touches(block) {
                collides = true;
            }
        }

        collides
    }

    fn reset_lock_delay(&mut self) {
        if self.lock_delay {
            self.drop_tick = 0.0;
        }
    }

    fn blocks(&self) -> Vec<Pos> {
        self.tetromino
            .blocks(self.rot)
            .iter()
            .map(|pos| *pos + self.pos)
            .collect()
    }
}

fn initial_pos() -> Pos {
    Pos::new(INITIAL_X, 0)
}
