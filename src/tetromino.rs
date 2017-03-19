extern crate rand;

use pos::Pos;
use block::draw_block;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::pixels::Color::RGB;
use sdl2::render::Renderer;

const NUM_TETROMINOES: usize = 7;
const NUM_ROTATIONS: i8 = 4;

pub const WIDTH: u8 = 4;
pub const HEIGHT: u8 = 4;

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

    pub fn peek(&self) -> &'static Tetromino {
        self.tetrominoes[self.index]
    }

    pub fn pop(&mut self) -> &'static Tetromino {
        let next = self.tetrominoes[self.index];

        self.index += 1;

        if self.index >= NUM_TETROMINOES {
            self.tetrominoes = Bag::random_sequence();
            self.index = 0;
        }

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
pub struct Rotation(i8);

impl Rotation {
    pub fn clockwise(&self) -> Rotation {
        let new = self.0 + 1;
        Rotation(((new % NUM_ROTATIONS) + NUM_ROTATIONS) % NUM_ROTATIONS)
    }

    pub fn anticlockwise(&self) -> Rotation {
        let new = self.0 - 1;
        Rotation(((new % NUM_ROTATIONS) + NUM_ROTATIONS) % NUM_ROTATIONS)
    }
}

impl Default for Rotation {
    fn default() -> Rotation {
        Rotation(0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tetromino {
    rotations: [[[bool; WIDTH as usize]; HEIGHT as usize]; NUM_ROTATIONS as usize],
    pub color: Color,
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

    pub fn draw(&self, renderer: &Renderer, rot: Rotation, pos: Pos) {
        for block in self.blocks(rot) {
            draw_block(renderer, pos + block, self.color);
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
                 [true, true, true, true],
                 [false, false, false, false],
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
                 [true, false, false, false],
                 [true, true, true, false],
                 [false, false, false, false]],
                [[false, false, false, false],
                 [false, true, true, false],
                 [false, true, false, false],
                 [false, true, false, false]],
                [[false, false, false, false],
                 [false, false, false, false],
                 [true, true, true, false],
                 [false, false, true, false]],
                [[false, false, false, false],
                 [false, true, false, false],
                 [false, true, false, false],
                 [true, true, false, false]]],
    color: RGB(0, 0, 255),
};

static L_TET: Tetromino = Tetromino {
    rotations: [[[false, false, false, false],
                 [false, false, true, false],
                 [true, true, true, false],
                 [false, false, false, false]],
                [[false, false, false, false],
                 [false, true, false, false],
                 [false, true, false, false],
                 [false, true, true, false]],
                [[false, false, false, false],
                 [false, false, false, false],
                 [true, true, true, false],
                 [true, false, false, false]],
                [[false, false, false, false],
                 [true, true, false, false],
                 [false, true, false, false],
                 [false, true, false, false]]],
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
                 [false, true, false, false],
                 [true, true, true, false],
                 [false, false, false, false]],
                [[false, false, false, false],
                 [false, true, false, false],
                 [false, true, true, false],
                 [false, true, false, false]],
                [[false, false, false, false],
                 [false, false, false, false],
                 [true, true, true, false],
                 [false, true, false, false]],
                [[false, false, false, false],
                 [false, true, false, false],
                 [true, true, false, false],
                 [false, true, false, false]]],
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
            let size = g.size() as u32;
            if g.gen_weighted_bool(size) {
                Bag::new()
            } else {
                let mut bag = Bag::arbitrary(g);
                bag.pop();
                bag
            }
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
