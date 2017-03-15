use tile::draw_tile;
use pos::Pos;

use sdl2::pixels::Color;
use sdl2::render::Renderer;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 24;

#[derive(Clone, Debug)]
pub struct Board([[Option<Color>; WIDTH]; HEIGHT]);

impl Board {
    pub fn new() -> Board {
        Board([[None; WIDTH]; HEIGHT])
    }

    pub fn touches(&self, pos: Pos) -> bool {
        out_bounds(pos) || self.0[pos.y() as usize][pos.x() as usize].is_some()
    }

    pub fn fill(&mut self, pos: Pos, color: Color) {
        self.0[pos.y() as usize][pos.x() as usize] = Some(color);
    }

    pub fn check_clear(&mut self) {
        for y in 0..HEIGHT {
            let mut clear = true;

            'check_clear: for cell in self.0[y].iter() {
                if cell.is_none() {
                    clear = false;
                    break 'check_clear;
                }
            }

            if clear {
                self.clear_row(y);
            }
        }
    }

    fn clear_row(&mut self, y: usize) {
        for yy in (1..y + 1).rev() {
            self.0[yy] = self.0[yy - 1];
        }

        for x in 0..WIDTH {
            self.0[0][x] = None;
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match *cell {
                    Some(color) => draw_tile(&renderer, Pos::new(x as isize, y as isize), color),
                    None => (),
                }
            }
        }
    }
}

fn out_bounds(pos: Pos) -> bool {
    pos.x() < 0 || pos.y() < 0 || pos.x() >= WIDTH as isize || pos.y() >= HEIGHT as isize
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen, TestResult};
    use std::mem;
    use std::ptr;
    use sdl2::pixels::Color;

    impl Arbitrary for Board {
        fn arbitrary<G: Gen>(g: &mut G) -> Board {
            unsafe {
                let mut array: [[Option<Color>; WIDTH]; HEIGHT] = mem::uninitialized();

                for row in array.iter_mut() {
                    for cell in row.iter_mut() {
                        let color = if g.gen() {
                            Some(Color::RGB(g.gen(), g.gen(), g.gen()))
                        } else {
                            None
                        };

                        ptr::write(cell, color);
                    }
                }

                Board(array)
            }
        }
    }

    fn in_bounds(pos: Pos) -> bool {
        !out_bounds(pos)
    }

    quickcheck! {

        fn a_new_board_is_empty(pos: Pos) -> TestResult {
            when!(in_bounds(pos));
            then!(!Board::new().touches(pos))
        }

        fn after_filling_a_space_it_is_filled(board: Board, pos: Pos, r: u8, g: u8, b: u8) -> TestResult {
            when!(in_bounds(pos));
            let color = Color::RGB(r, g, b);
            let mut board = board;
            board.fill(pos, color);
            then!(board.touches(pos))
        }

        fn after_filling_a_space_no_other_space_changes(board: Board, pos1: Pos, pos2: Pos, r: u8, g: u8, b: u8) -> TestResult {
            when!(pos1 != pos2);
            when!(in_bounds(pos1));

            let color = Color::RGB(r, g, b);
            let mut board = board;

            let touches_before = board.touches(pos2);
            board.fill(pos1, color);
            let touches_after = board.touches(pos2);
            then!(touches_before == touches_after)
        }

        fn after_clearing_a_row_the_top_row_is_empty(board: Board, x: isize, y: usize) -> TestResult {
            when!(in_bounds(Pos::new(x, y as isize)));
            let mut board = board;
            board.clear_row(y);
            then!(!board.touches(Pos::new(x, 0)))
        }

        fn after_clearing_a_row_nothing_under_it_is_changed(board: Board, y: usize, under: Pos) -> TestResult {
            when!(in_bounds(under));
            when!(under.y() > y as isize);

            let mut board = board;

            let before = board.touches(under);
            board.clear_row(y);
            let after = board.touches(under);
            then!(before == after)
        }

        fn after_clearing_a_row_everything_above_it_shifts_down(board: Board, y: usize, above: Pos) -> TestResult {
            when!(y < HEIGHT);
            when!(!out_bounds(above));
            when!(above.y() < y as isize);

            let mut board = board;

            let before = board.touches(above);
            board.clear_row(y);
            let after = board.touches(above.down());
            then!(before == after)
        }
    }
}
