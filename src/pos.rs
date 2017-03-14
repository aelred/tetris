use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        Pos { x: x, y: y }
    }

    pub fn x(self) -> usize {
        self.x
    }

    pub fn y(self) -> usize {
        self.y
    }

    pub fn left(self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn up(self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }

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
    use super::*;
    use quickcheck::{Arbitrary, Gen};

    const ID: Pos = Pos { x: 0, y: 0 };

    impl Arbitrary for Pos {
        fn arbitrary<G: Gen>(g: &mut G) -> Pos {
            // We don't need to worry about positions that might overflow
            Pos {
                x: g.gen_range(0, 30),
                y: g.gen_range(0, 30),
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
    }
}
