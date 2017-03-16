use tile::draw_tile;
use tile::draw_border;
use pos::Pos;

use sdl2::pixels::Color;
use sdl2::render::Renderer;

pub const WIDTH: u8 = 10;
pub const HEIGHT: u8 = 24;

#[derive(Clone, Debug)]
pub struct Board([[Option<Color>; WIDTH as usize]; HEIGHT as usize]);

impl Board {
    pub fn new() -> Board {
        Board([[None; WIDTH as usize]; HEIGHT as usize])
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

            'check_clear: for cell in &self.0[y as usize] {
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

    fn clear_row(&mut self, y: u8) {
        for yy in (1..y as usize + 1).rev() {
            self.0[yy] = self.0[yy - 1];
        }

        for x in 0..WIDTH as usize {
            self.0[0][x] = None;
        }
    }

    pub fn draw(&self, pos: Pos, renderer: &Renderer) {
        draw_border(renderer, pos, pos + Pos::new(WIDTH as i8, HEIGHT as i8));

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Some(color) = self.0[y as usize][x as usize] {
                    let cell_pos = Pos::new(x as i8, y as i8);
                    draw_tile(renderer, cell_pos + pos, color)
                }
            }
        }
    }
}

fn out_bounds(pos: Pos) -> bool {
    pos.x() < 0 || pos.y() < 0 || pos.x() >= WIDTH as i8 || pos.y() >= HEIGHT as i8
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
                let mut array: [[Option<Color>; WIDTH as usize]; HEIGHT as usize] =
                    mem::uninitialized();

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

        fn after_clearing_a_row_the_top_row_is_empty(board: Board, x: i8, y: u8) -> TestResult {
            when!(in_bounds(Pos::new(x, y as i8)));
            let mut board = board;
            board.clear_row(y);
            then!(!board.touches(Pos::new(x, 0)))
        }

        fn after_clearing_a_row_nothing_under_it_is_changed(board: Board, y: u8, under: Pos) -> TestResult {
            when!(in_bounds(under));
            when!(under.y() > y as i8);

            let mut board = board;

            let before = board.touches(under);
            board.clear_row(y);
            let after = board.touches(under);
            then!(before == after)
        }

        fn after_clearing_a_row_everything_above_it_shifts_down(board: Board, y: u8, above: Pos) -> TestResult {
            when!(y < HEIGHT);
            when!(!out_bounds(above));
            when!(above.y() < y as i8);

            let mut board = board;

            let before = board.touches(above);
            board.clear_row(y);
            let after = board.touches(above.down());
            then!(before == after)
        }
    }
}
