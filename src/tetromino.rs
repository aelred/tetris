extern crate rand;

use pos::Pos;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::pixels::Color::RGB;

const NUM_TETROMINOES: usize = 7;
const WIDTH: usize = 4;
const HEIGHT: usize = 4;
const NUM_ROTATIONS: usize = 4;

#[derive(Debug, Clone)]
pub struct Bag {
    tetrominoes: [&'static Tetromino; NUM_TETROMINOES],
    index: usize,
}

impl Bag {
    pub fn new() -> Bag {
        Bag {
            tetrominoes: Bag::random_sequence(),
            index: 0,
        }
    }

    pub fn next(&mut self) -> &'static Tetromino {
        if self.index >= NUM_TETROMINOES {
            self.tetrominoes = Bag::random_sequence();
            self.index = 0;
        }

        let next = self.tetrominoes[self.index];
        self.index += 1;
        next
    }

    fn random_sequence() -> [&'static Tetromino; NUM_TETROMINOES] {
        let mut rng = rand::thread_rng();
        let mut sequence = TETROMINOES;
        rng.shuffle(&mut sequence);
        sequence
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rotation(usize);

impl Rotation {
    pub fn new() -> Rotation {
        Rotation(0)
    }

    pub fn rotate(&self) -> Rotation {
        Rotation((self.0 + 1) % NUM_ROTATIONS)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tetromino {
    rotations: [[[bool; HEIGHT]; WIDTH]; NUM_ROTATIONS],
    pub color: Color,
}

impl Tetromino {
    pub fn each_cell<F>(&self, rot: Rotation, mut f: F)
        where F: FnMut(Pos) -> ()
    {
        for (x, col) in self.rotations[rot.0].iter().enumerate() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;
    use quickcheck::Gen;

    impl Arbitrary for Bag {
        fn arbitrary<G: Gen>(g: &mut G) -> Bag {
            let mut bag = Bag::new();
            for _ in 0..g.gen_range(0, NUM_TETROMINOES * 2) {
                bag.next();
            }
            bag
        }
    }

    impl Arbitrary for Rotation {
        fn arbitrary<G: Gen>(g: &mut G) -> Rotation {
            if g.gen() {
                Rotation::arbitrary(g).rotate()
            } else {
                Rotation::new()
            }
        }
    }

    impl Arbitrary for &'static Tetromino {
        fn arbitrary<G: Gen>(g: &mut G) -> &'static Tetromino {
            g.choose(&TETROMINOES).unwrap()
        }
    }

    quickcheck! {
        fn bag_always_returns_a_valid_tetromino(bag: Bag) -> bool {
            let mut bag = bag;
            let tetromino = bag.next();
            TETROMINOES.iter().any(|t| *t == tetromino)
        }

        fn bag_never_returns_same_tetromino_three_times(bag: Bag) -> bool {
            let mut bag = bag;
            let first = bag.next();
            let second = bag.next();
            let third = bag.next();
            !(first == second && second == third)
        }

        fn rotation_four_times_is_identity(rot: Rotation) -> bool {
            rot == rot.rotate().rotate().rotate().rotate()
        }
    }
}
