use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Pos {
        Pos { x: x, y: y }
    }

    pub fn x(self) -> isize {
        self.x
    }

    pub fn y(self) -> isize {
        self.y
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
            let id = Pos {x: 0, y: 0};
            p + id == p && id + p == p
        }
    }
}
