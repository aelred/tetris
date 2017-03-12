use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
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
