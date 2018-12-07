use lazy_static::lazy_static;

use crate::Board;
use crate::Pos;
use crate::Rotation;
use crate::Shape;

/// The initial X position that a piece spawns on the board.
const INITIAL_X: i16 = Board::WIDTH as i16 / 2 - 2;

/// A tetromino piece in play.
#[derive(Debug)]
pub struct Piece {
    /// The shape of the piece.
    pub shape: Shape,

    /// The orientation of the piece.
    pub rot: Rotation,

    /// The position of the piece on the board.
    pub pos: Pos,
}

impl Piece {
    /// Create a new piece at the initial position and rotation.
    pub fn new(shape: Shape) -> Piece {
        Piece {
            shape,
            rot: Rotation::default(),
            pos: *INITIAL_POS,
        }
    }

    /// Rotate the piece clockwise.
    pub fn rotate_clockwise(&mut self) {
        self.rot = self.rot.clockwise();
    }

    /// Rotate the piece anticlockwise.
    pub fn rotate_anticlockwise(&mut self) {
        self.rot = self.rot.anticlockwise();
    }

    /// Move the piece one space left.
    pub fn left(&mut self) {
        self.pos = self.pos.left();
    }

    /// Move the piece one space right.
    pub fn right(&mut self) {
        self.pos = self.pos.right();
    }

    /// Move the piece one space up.
    pub fn up(&mut self) {
        self.pos = self.pos.up();
    }

    /// Move the piece one space down.
    pub fn down(&mut self) {
        self.pos = self.pos.down();
    }

    /// Get all the blocks that make up the piece at the current position and rotation.
    pub fn blocks(&self) -> Vec<Pos> {
        self.shape
            .blocks(self.rot)
            .iter()
            .map(|pos| *pos + self.pos)
            .collect()
    }
}

lazy_static! {
    /// The initial position that a piece spawns on the board.
    static ref INITIAL_POS: Pos = Pos::new(INITIAL_X, 0);
}
