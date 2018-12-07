use std::ops::Add;

/// A 2D integer position or vector.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    /// Create a new position.
    pub fn new(x: i16, y: i16) -> Pos {
        Pos { x, y }
    }

    /// Return the x component of the position.
    pub fn x(self) -> i16 {
        self.x
    }

    /// Return the y component of the position.
    pub fn y(self) -> i16 {
        self.y
    }

    /// Return a position one unit to the left.
    pub fn left(self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }

    /// Return a position one unit to the right.
    pub fn right(self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }

    /// Return a position one unit up.
    pub fn up(self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }

    /// Return a position one unit down.
    pub fn down(self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen, quickcheck};

    use super::*;

    const ID: Pos = Pos { x: 0, y: 0 };

    impl Arbitrary for Pos {
        fn arbitrary<G: Gen>(g: &mut G) -> Pos {
            // We don't need to worry about positions that might overflow
            Pos {
                x: g.gen_range(-30, 30),
                y: g.gen_range(-30, 30),
            }
        }
    }

    quickcheck! {
        fn add_is_commutative(a: Pos, b: Pos) -> bool {
            a + b == b + a
        }

        fn add_is_associative(a: Pos, b: Pos, c: Pos) -> bool {
            (a + b) + c == a + (b + c)
        }

        fn add_has_identity_element(p: Pos) -> bool {
            p + ID == p && ID + p == p
        }

        fn when_moving_left_piece_is_one_space_left(p: Pos) -> bool {
            p.left().x() == p.x() - 1
        }

        fn when_moving_left_piece_has_same_y(p: Pos) -> bool {
            p.left().y() == p.y()
        }

        fn when_moving_right_piece_is_one_space_right(p: Pos) -> bool {
            p.right().x() == p.x() + 1
        }

        fn when_moving_right_piece_has_same_y(p: Pos) -> bool {
            p.right().y() == p.y()
        }
    }
}
