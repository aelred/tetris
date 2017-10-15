extern crate rand;

use pos::Pos;
use draw::Drawer;

use std::fmt;
use rand::XorShiftRng;
use rand::Rng;

const NUM_TETROMINOES: usize = 7;
const NUM_ROTATIONS: i8 = 4;

pub const WIDTH: u8 = 4;
pub const HEIGHT: u8 = 4;

#[derive(Clone)]
pub struct Bag {
    tetrominoes: [&'static Tetromino; NUM_TETROMINOES],
    index: usize,
    rng: XorShiftRng,
}

impl fmt::Debug for Bag {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Bag")
            .field("tetrominoes", &self.tetrominoes)
            .field("index", &self.index)
            .field("rng", &"<rng>")
            .finish()
    }
}

impl Bag {
    pub fn new(mut rng: XorShiftRng) -> Bag {
        Bag {
            tetrominoes: Bag::random_sequence(&mut rng),
            index: 0,
            rng,
        }
    }

    pub fn peek(&self) -> &'static Tetromino {
        self.tetrominoes[self.index]
    }

    pub fn pop(&mut self) -> &'static Tetromino {
        let next = self.tetrominoes[self.index];

        self.index += 1;

        if self.index >= NUM_TETROMINOES {
            self.tetrominoes = Bag::random_sequence(&mut self.rng);
            self.index = 0;
        }

        next
    }

    fn random_sequence<R: Rng>(rng: &mut R) -> [&'static Tetromino; NUM_TETROMINOES] {
        let mut sequence = TETROMINOES;

        // This is inlined from `Rng::shuffle`.
        // We do this so we can cast `i` into a `u8`, meaning the shuffle is reliable regardless
        // of differences in `usize`.
        // This allows us to replay a game on a different machine and get the same result.
        // This cast is safe because the sequence of tetrominos will definitely fit in a u8.
        let mut i = sequence.len() as u8;
        while i >= 2 {
            // invariant: elements with index >= i have been locked in place.
            i -= 1;
            // lock element i in place.
            sequence.swap(i as usize, rng.gen_range(0, i + 1) as usize);
        }

        sequence
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rotation(i8);

impl Rotation {
    pub fn clockwise(&self) -> Rotation {
        Rotation(modulo(self.0 + 1, NUM_ROTATIONS))
    }

    pub fn anticlockwise(&self) -> Rotation {
        Rotation(modulo(self.0 - 1, NUM_ROTATIONS))
    }
}

/// Calculates modulo. This is distinct from `%`, which calculates the remainder.
fn modulo(x: i8, y: i8) -> i8 {
    ((x % y) + y) % y
}

impl Default for Rotation {
    fn default() -> Rotation {
        Rotation(0)
    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum TetColor {
    O,
    I,
    J,
    L,
    S,
    T,
    Z,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tetromino {
    rotations: [[[bool; WIDTH as usize]; HEIGHT as usize]; NUM_ROTATIONS as usize],
    pub color: TetColor,
}

impl Tetromino {
    pub fn blocks(&self, rot: Rotation) -> Vec<Pos> {
        let mut blocks = Vec::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.rotations[rot.0 as usize][y as usize][x as usize] {
                    blocks.push(Pos::new(x as i16, y as i16));
                };
            }
        }

        blocks
    }

    pub fn draw(&self, drawer: &mut Drawer, rot: Rotation, pos: Pos) {
        for block in self.blocks(rot) {
            drawer.draw_block(pos + block, self.color);
        }
    }
}

static TETROMINOES: [&'static Tetromino; NUM_TETROMINOES] =
    [&O_TET, &I_TET, &J_TET, &L_TET, &S_TET, &T_TET, &Z_TET];

static O_TET: Tetromino = Tetromino {
    rotations: [
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
    ],
    color: TetColor::O,
};

static I_TET: Tetromino = Tetromino {
    rotations: [
        [
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, true, false],
            [false, false, true, false],
            [false, false, true, false],
            [false, false, true, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
    ],
    color: TetColor::I,
};

static J_TET: Tetromino = Tetromino {
    rotations: [
        [
            [false, false, false, false],
            [true, false, false, false],
            [true, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, false],
            [false, false, true, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [true, true, false, false],
        ],
    ],
    color: TetColor::J,
};

static L_TET: Tetromino = Tetromino {
    rotations: [
        [
            [false, false, false, false],
            [false, false, true, false],
            [true, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, true, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, false],
            [true, false, false, false],
        ],
        [
            [false, false, false, false],
            [true, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
    ],
    color: TetColor::L,
};

static S_TET: Tetromino = Tetromino {
    rotations: [
        [
            [false, false, false, false],
            [false, true, true, false],
            [true, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [false, true, true, false],
            [false, false, true, false],
            [false, false, false, false],
        ],
        [
            [false, true, true, false],
            [true, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [false, true, true, false],
            [false, false, true, false],
            [false, false, false, false],
        ],
    ],
    color: TetColor::S,
};

static T_TET: Tetromino = Tetromino {
    rotations: [
        [
            [false, false, false, false],
            [false, true, false, false],
            [true, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, true, false],
            [false, true, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, false],
            [false, true, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [true, true, false, false],
            [false, true, false, false],
        ],
    ],
    color: TetColor::T,
};

static Z_TET: Tetromino = Tetromino {
    rotations: [
        [
            [false, false, false, false],
            [true, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [true, true, false, false],
            [true, false, false, false],
            [false, false, false, false],
        ],
        [
            [true, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, true, false],
            [false, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    ],
    color: TetColor::Z,
};

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Bag {
        fn arbitrary<G: Gen>(g: &mut G) -> Bag {
            let size = g.size() as u32;
            let mut bag = Bag::new(rand::random());
            for _ in 0..size {
                bag.pop();
            }
            bag
        }
    }

    impl Arbitrary for Rotation {
        fn arbitrary<G: Gen>(g: &mut G) -> Rotation {
            if g.gen() {
                Rotation::default()
            } else if g.gen() {
                Rotation::arbitrary(g).clockwise()
            } else {
                Rotation::arbitrary(g).anticlockwise()
            }
        }
    }

    impl Arbitrary for &'static Tetromino {
        fn arbitrary<G: Gen>(g: &mut G) -> &'static Tetromino {
            g.choose(&TETROMINOES).unwrap()
        }
    }

    impl Arbitrary for TetColor {
        fn arbitrary<G: Gen>(g: &mut G) -> TetColor {
            *g.choose(
                &[
                    TetColor::O,
                    TetColor::I,
                    TetColor::J,
                    TetColor::L,
                    TetColor::S,
                    TetColor::T,
                    TetColor::Z,
                ],
            ).unwrap()
        }
    }

    #[test]
    fn bag_always_returns_exact_result_for_same_seed() {
        let rng = XorShiftRng::new_unseeded();
        let mut bag = Bag::new(rng);

        let mut vec = Vec::new();

        for _ in 0..10 {
            vec.push(bag.pop().color);
        }

        assert_eq!(
            vec![
                TetColor::J,
                TetColor::L,
                TetColor::S,
                TetColor::T,
                TetColor::Z,
                TetColor::O,
                TetColor::I,
                TetColor::Z,
                TetColor::L,
                TetColor::J,
            ],
            vec
        );
    }

    quickcheck! {
        fn bag_always_returns_a_valid_tetromino(bag: Bag) -> bool {
            let mut bag = bag;
            let tetromino = bag.pop();
            TETROMINOES.iter().any(|t| *t == tetromino)
        }

        fn bag_never_returns_same_tetromino_three_times(bag: Bag) -> bool {
            let mut bag = bag;
            let first = bag.pop();
            let second = bag.pop();
            let third = bag.pop();
            !(first == second && second == third)
        }

        fn bag_always_returns_same_piece_within_thirteen_times(bag: Bag) -> bool {
            let mut bag = bag;
            let initial = bag.pop();
            for _ in 0..13 {
                if bag.pop() == initial {
                    return true;
                }
            }
            false
        }

        fn bag_always_returns_the_same_result_with_the_same_seed(x: u32, y: u32, z: u32, w: u32) -> bool {
            use rand::SeedableRng;

            let seed = [x, y, z, w];

            let rng1 = XorShiftRng::from_seed(seed);
            let mut bag1 = Bag::new(rng1);

            let rng2 = XorShiftRng::from_seed(seed);
            let mut bag2 = Bag::new(rng2);

            for _ in 0..100 {
                if bag1.pop() != bag2.pop() {
                    return false;
                }
            }

            return true;
        }

        fn peek_has_same_result_as_pop(bag: Bag) -> bool {
            let mut bag = bag;
            bag.peek() == bag.pop()
        }

        fn clockwise_rotation_once_is_different(rot: Rotation) -> bool {
            rot != rot.clockwise()
        }

        fn clockwise_rotation_twice_is_different(rot: Rotation) -> bool {
            rot != rot.clockwise().clockwise()
        }

        fn clockwise_rotation_thrice_is_different(rot: Rotation) -> bool {
            rot != rot.clockwise().clockwise().clockwise()
        }

        fn clockwise_rotation_four_times_is_identity(rot: Rotation) -> bool {
            rot == rot.clockwise().clockwise().clockwise().clockwise()
        }

        fn anticlockwise_rotation_four_times_is_identity(rot: Rotation) -> bool {
            rot == rot.anticlockwise().anticlockwise().anticlockwise().anticlockwise()
        }

        fn anticlockwise_is_inverse_of_clockwise(rot: Rotation) -> bool {
            rot.clockwise().anticlockwise() == rot
        }

        fn clockwise_is_inverse_of_anticlockwise(rot: Rotation) -> bool {
            rot.anticlockwise().clockwise() == rot
        }
    }
}
