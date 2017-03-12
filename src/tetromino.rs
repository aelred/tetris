extern crate rand;

use pos::Pos;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::pixels::Color::RGB;

const NUM_TETROMINOES: usize = 7;
const TETROMINO_WIDTH: usize = 4;
const TETROMINO_HEIGHT: usize = 4;
pub const NUM_ROTATIONS: usize = 4;

pub struct Tetromino {
    rotations: [[[bool; TETROMINO_HEIGHT]; TETROMINO_WIDTH]; NUM_ROTATIONS],
    pub color: Color,
}

impl Tetromino {
    pub fn random() -> &'static Tetromino {
        let mut rng = rand::thread_rng();
        TETROMINOES[rng.gen_range(0, NUM_TETROMINOES)]
    }

    pub fn each_cell<F>(&self, rot_index: usize, mut f: F)
        where F: FnMut(Pos) -> ()
    {
        for (x, col) in self.rotations[rot_index].iter().enumerate() {
            for (y, cell) in col.iter().enumerate() {
                if *cell {
                    f(Pos {
                          x: x as isize,
                          y: y as isize,
                      })
                };
            }
        }
    }
}

static TETROMINOES: [&'static Tetromino; NUM_TETROMINOES] = [&O_TET, &I_TET, &J_TET, &L_TET,
                                                             &S_TET, &T_TET, &Z_TET];

static O_TET: Tetromino = Tetromino {
    rotations: [[[false, false, false, false],
                 [false, true, true, false],
                 [false, true, true, false],
                 [false, false, false, false]],
                [[false, false, false, false],
                 [false, true, true, false],
                 [false, true, true, false],
                 [false, false, false, false]],
                [[false, false, false, false],
                 [false, true, true, false],
                 [false, true, true, false],
                 [false, false, false, false]],
                [[false, false, false, false],
                 [false, true, true, false],
                 [false, true, true, false],
                 [false, false, false, false]]],
    color: RGB(255, 255, 0),
};

static I_TET: Tetromino = Tetromino {
    rotations: [[[false, false, false, false],
                 [false, false, false, false],
                 [true, true, true, true],
                 [false, false, false, false]],
                [[false, false, true, false],
                 [false, false, true, false],
                 [false, false, true, false],
                 [false, false, true, false]],
                [[false, false, false, false],
                 [false, false, false, false],
                 [true, true, true, true],
                 [false, false, false, false]],
                [[false, true, false, false],
                 [false, true, false, false],
                 [false, true, false, false],
                 [false, true, false, false]]],
    color: RGB(0, 255, 255),
};

static J_TET: Tetromino = Tetromino {
    rotations: [[[false, false, false, false],
                 [true, true, true, false],
                 [false, false, true, false],
                 [false, false, false, false]],
                [[false, true, false, false],
                 [false, true, false, false],
                 [true, true, false, false],
                 [false, false, false, false]],
                [[true, false, false, false],
                 [true, true, true, false],
                 [false, false, false, false],
                 [false, false, false, false]],
                [[false, true, true, false],
                 [false, true, false, false],
                 [false, true, false, false],
                 [false, false, false, false]]],
    color: RGB(0, 0, 255),
};

static L_TET: Tetromino = Tetromino {
    rotations: [[[false, false, false, false],
                 [true, true, true, false],
                 [true, false, false, false],
                 [false, false, false, false]],
                [[true, true, false, false],
                 [false, true, false, false],
                 [false, true, false, false],
                 [false, false, false, false]],
                [[false, false, true, false],
                 [true, true, true, false],
                 [false, false, false, false],
                 [false, false, false, false]],
                [[false, true, false, false],
                 [false, true, false, false],
                 [false, true, true, false],
                 [false, false, false, false]]],
    color: RGB(255, 165, 0),
};

static S_TET: Tetromino = Tetromino {
    rotations: [[[false, false, false, false],
                 [false, true, true, false],
                 [true, true, false, false],
                 [false, false, false, false]],
                [[false, true, false, false],
                 [false, true, true, false],
                 [false, false, true, false],
                 [false, false, false, false]],
                [[false, true, true, false],
                 [true, true, false, false],
                 [false, false, false, false],
                 [false, false, false, false]],
                [[false, true, false, false],
                 [false, true, true, false],
                 [false, false, true, false],
                 [false, false, false, false]]],
    color: RGB(0, 255, 0),
};

static T_TET: Tetromino = Tetromino {
    rotations: [[[false, false, false, false],
                 [true, true, true, false],
                 [false, true, false, false],
                 [false, false, false, false]],
                [[false, true, false, false],
                 [true, true, false, false],
                 [false, true, false, false],
                 [false, false, false, false]],
                [[false, true, false, false],
                 [true, true, true, false],
                 [false, false, false, false],
                 [false, false, false, false]],
                [[false, true, false, false],
                 [false, true, true, false],
                 [false, true, false, false],
                 [false, false, false, false]]],
    color: RGB(255, 0, 255),
};

static Z_TET: Tetromino = Tetromino {
    rotations: [[[false, false, false, false],
                 [true, true, false, false],
                 [false, true, true, false],
                 [false, false, false, false]],
                [[false, true, false, false],
                 [true, true, false, false],
                 [true, false, false, false],
                 [false, false, false, false]],
                [[true, true, false, false],
                 [false, true, true, false],
                 [false, false, false, false],
                 [false, false, false, false]],
                [[false, false, true, false],
                 [false, true, true, false],
                 [false, true, false, false],
                 [false, false, false, false]]],
    color: RGB(255, 0, 0),
};
