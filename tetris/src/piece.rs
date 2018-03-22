use tetromino::Tetromino;
use tetromino::Rotation;
use pos::Pos;
use board::WIDTH;

const INITIAL_X: i16 = WIDTH as i16 / 2 - 2;

#[derive(Debug)]
pub struct Piece {
    pub tetromino: &'static Tetromino,
    pub rot: Rotation,
    pub pos: Pos,
}

impl Piece {
    pub fn new(tetromino: &'static Tetromino) -> Piece {
        Piece {
            tetromino,
            rot: Rotation::default(),
            pos: *INITIAL_POS,
        }
    }

    pub fn rotate_clockwise(&mut self) {
        self.rot = self.rot.clockwise();
    }

    pub fn rotate_anticlockwise(&mut self) {
        self.rot = self.rot.anticlockwise();
    }

    pub fn left(&mut self) {
        self.pos = self.pos.left();
    }

    pub fn right(&mut self) {
        self.pos = self.pos.right();
    }

    pub fn up(&mut self) {
        self.pos = self.pos.up();
    }

    pub fn down(&mut self) {
        self.pos = self.pos.down();
    }

    pub fn blocks(&self) -> Vec<Pos> {
        self.tetromino
            .blocks(self.rot)
            .iter()
            .map(|pos| *pos + self.pos)
            .collect()
    }
}

lazy_static! {
    static ref INITIAL_POS: Pos = Pos::new(INITIAL_X, 0);
}