use pos::Pos;
use draw::Drawer;

use sdl2::pixels::Color;

pub const WIDTH: u8 = 10;
pub const HEIGHT: u8 = 24;

pub const HIDE_ROWS: u8 = 4;

#[derive(Clone, Debug)]
pub struct Board {
    grid: [[Option<Color>; WIDTH as usize]; HEIGHT as usize],
}

pub struct FillResult {
    pub is_game_over: bool,
    pub lines_cleared: u32,
}

impl Board {
    pub fn new() -> Board {
        Board { grid: [[None; WIDTH as usize]; HEIGHT as usize] }
    }

    pub fn touches(&self, pos: Pos) -> bool {
        out_bounds(pos) || self.grid[pos.y() as usize][pos.x() as usize].is_some()
    }

    pub fn fill(&mut self, cells: Vec<Pos>, color: Color) -> FillResult {
        let mut is_game_over = true;

        for cell in cells {
            if cell.y() > HIDE_ROWS as i16 {
                is_game_over = false;
            }
            self.fill_pos(cell, color);
        }

        FillResult {
            is_game_over: is_game_over,
            lines_cleared: self.check_clear(),
        }
    }

    fn fill_pos(&mut self, pos: Pos, color: Color) {
        assert!(!out_bounds(pos));
        self.grid[pos.y() as usize][pos.x() as usize] = Some(color);
    }

    fn check_clear(&mut self) -> u32 {
        let mut lines_cleared = 0;

        for y in 0..HEIGHT {
            let mut clear = true;

            'check_clear: for cell in &self.grid[y as usize] {
                if cell.is_none() {
                    clear = false;
                    break 'check_clear;
                }
            }

            if clear {
                self.clear_row(y);
                lines_cleared += 1;
            }
        }

        lines_cleared
    }

    fn clear_row(&mut self, y: u8) {
        for yy in (1..y as usize + 1).rev() {
            self.grid[yy] = self.grid[yy - 1];
        }

        for x in 0..WIDTH as usize {
            self.grid[0][x] = None;
        }
    }

    pub fn draw_border(&self, drawer: &mut Drawer) {
        drawer.draw_border(Pos::new(WIDTH as i16, (HEIGHT - HIDE_ROWS) as i16));
    }

    pub fn draw(&self, drawer: &mut Drawer) {
        for y in HIDE_ROWS..HEIGHT {
            for x in 0..WIDTH {
                if let Some(color) = self.grid[y as usize][x as usize] {
                    let y = y - HIDE_ROWS;
                    let cell_pos = Pos::new(x as i16, y as i16);
                    drawer.draw_block(cell_pos, color)
                }
            }
        }
    }
}

fn out_bounds(pos: Pos) -> bool {
    pos.x() < 0 || pos.y() < 0 || pos.x() >= WIDTH as i16 || pos.y() >= HEIGHT as i16
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

                Board { grid: array }
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
            board.fill_pos(pos, color);
            then!(board.touches(pos))
        }

        fn after_filling_a_space_no_other_space_changes(board: Board, pos1: Pos, pos2: Pos, r: u8, g: u8, b: u8) -> TestResult {
            when!(pos1 != pos2);
            when!(in_bounds(pos1));

            let color = Color::RGB(r, g, b);
            let mut board = board;

            let touches_before = board.touches(pos2);
            board.fill_pos(pos1, color);
            let touches_after = board.touches(pos2);
            then!(touches_before == touches_after)
        }

        fn after_clearing_a_row_the_top_row_is_empty(board: Board, x: i16, y: u8) -> TestResult {
            when!(in_bounds(Pos::new(x, y as i16)));
            let mut board = board;
            board.clear_row(y);
            then!(!board.touches(Pos::new(x, 0)))
        }

        fn after_clearing_a_row_nothing_under_it_is_changed(board: Board, y: u8, under: Pos) -> TestResult {
            when!(in_bounds(under));
            when!(under.y() > y as i16);

            let mut board = board;

            let before = board.touches(under);
            board.clear_row(y);
            let after = board.touches(under);
            then!(before == after)
        }

        fn after_clearing_a_row_everything_above_it_shifts_down(board: Board, y: u8, above: Pos) -> TestResult {
            when!(y < HEIGHT);
            when!(!out_bounds(above));
            when!(above.y() < y as i16);

            let mut board = board;

            let before = board.touches(above);
            board.clear_row(y);
            let after = board.touches(above.down());
            then!(before == after)
        }
    }
}
