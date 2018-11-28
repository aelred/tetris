use pos::Pos;
use shape::ShapeColor;
use piece::Piece;

pub const WIDTH: u8 = 10;

/// Height of the playable board in cells - note that some of the top-most cells are not visible in
/// play, indicated by `HIDE_ROWS`.
pub const HEIGHT: u8 = 24;

/// Number of rows at the top of the board that are not visible. This is where new pieces are
/// spawned.
pub const HIDE_ROWS: u8 = 4;

/// Number of visible rows, based on total height and number of hidden rows.
pub const VISIBLE_ROWS: u8 = HEIGHT - HIDE_ROWS;

/// The board state, describing which cells are full and what colour tetromino they were filled
/// with.
#[derive(Clone, Debug)]
pub struct Board {
    pub grid: [[Option<ShapeColor>; WIDTH as usize]; HEIGHT as usize],
}

/// The result of calling [`Board::lock_piece`](struct.Board.html#method.lock_piece), which is
/// called when locking a piece. Indicates if this caused a game over, or if any lines were cleared.
pub struct FillResult {
    pub is_game_over: bool,
    pub lines_cleared: u32,
}

impl Default for Board {
    /// Returns an empty board.
    fn default() -> Self {
        Board {
            grid: [[None; WIDTH as usize]; HEIGHT as usize],
        }
    }
}

impl Board {
    /// Returns if this position on the board is taken or out of bounds.
    pub fn touches(&self, pos: Pos) -> bool {
        out_bounds(pos) || self.grid[pos.y() as usize][pos.x() as usize].is_some()
    }

    /// Lock a piece, attaching it to the board permanently and potentially clearing some rows.
    ///
    /// This can cause a game over if the piece is locked above the visible playing area, which
    /// will be indicated in the return value.
    pub fn lock_piece(&mut self, piece: Piece) -> FillResult {
        let mut is_game_over = true;

        for cell in piece.blocks() {
            if cell.y() > i16::from(HIDE_ROWS) {
                is_game_over = false;
            }
            self.fill_pos(cell, piece.shape.color);
        }

        FillResult {
            is_game_over,
            lines_cleared: self.clear_full_rows(),
        }
    }

    /// Fill a single position on the board.
    ///
    /// # Panics
    /// Panics if the position is out of bounds.
    fn fill_pos(&mut self, pos: Pos, color: ShapeColor) {
        assert!(!out_bounds(pos));
        self.grid[pos.y() as usize][pos.x() as usize] = Some(color);
    }

    /// Clear any full rows and return the number of rows cleared.
    fn clear_full_rows(&mut self) -> u32 {
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

    /// Clear the given row.
    fn clear_row(&mut self, y: u8) {
        for yy in (1..=y as usize).rev() {
            self.grid[yy] = self.grid[yy - 1];
        }

        for x in 0..WIDTH as usize {
            self.grid[0][x] = None;
        }
    }
}

/// Return whether the given position is out of bounds of the board (including hidden rows).
fn out_bounds(pos: Pos) -> bool {
    pos.x() < 0 || pos.y() < 0 || pos.x() >= i16::from(WIDTH) || pos.y() >= i16::from(HEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen, TestResult};
    use std::mem;
    use std::ptr;

    impl Arbitrary for Board {
        fn arbitrary<G: Gen>(g: &mut G) -> Board {
            unsafe {
                let mut array: [[Option<ShapeColor>; WIDTH as usize];
                                   HEIGHT as usize] = mem::uninitialized();

                for row in array.iter_mut() {
                    for cell in row.iter_mut() {
                        ptr::write(cell, Option::arbitrary(g));
                    }
                }

                Board { grid: array }
            }
        }
    }

    /// Represents only positions that are within the bounds of the board, such that
    /// `out_bounds(pos.0)` is always `false`.
    ///
    /// This speeds up property tests, because they do not need to generate and discard lots of
    /// out-of-bounds positions.
    #[derive(Clone, Debug)]
    struct InBoundsPos(Pos);

    impl Arbitrary for InBoundsPos {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            InBoundsPos(Pos::new(
                g.gen_range(0, WIDTH as i16),
                g.gen_range(0, HEIGHT as i16),
            ))
        }
    }

    quickcheck! {

        fn a_new_board_is_empty(pos: InBoundsPos) -> bool {
            !Board::default().touches(pos.0)
        }

        fn after_filling_a_space_it_is_filled(
            board: Board, pos: InBoundsPos, col: ShapeColor) -> bool {
            let pos = pos.0;
            let mut board = board;
            board.fill_pos(pos, col);
            board.touches(pos)
        }

        fn after_filling_a_space_no_other_space_changes(
            board: Board, pos1: InBoundsPos, pos2: Pos, col: ShapeColor) -> TestResult {

            let pos1 = pos1.0;
            let mut board = board;

            when!(pos1 != pos2);

            let touches_before = board.touches(pos2);
            board.fill_pos(pos1, col);
            let touches_after = board.touches(pos2);
            then!(touches_before == touches_after)
        }

        fn after_clearing_a_row_the_top_row_is_empty(board: Board, pos: InBoundsPos) -> bool {
            let pos = pos.0;
            let mut board = board;
            board.clear_row(pos.y() as u8);
            !board.touches(Pos::new(pos.x(), 0))
        }

        fn after_clearing_a_row_nothing_under_it_is_changed(
            board: Board, y: u8, under: InBoundsPos) -> TestResult {

            let under = under.0;
            let mut board = board;

            when!(under.y() > y as i16);

            let before = board.touches(under);
            board.clear_row(y);
            let after = board.touches(under);
            then!(before == after)
        }

        fn after_clearing_a_row_everything_above_it_shifts_down(
            board: Board, y: u8, above: InBoundsPos) -> TestResult {

            let above = above.0;
            let mut board = board;

            when!(y < HEIGHT);
            when!(above.y() < y as i16);

            let before = board.touches(above);
            board.clear_row(y);
            let after = board.touches(above.down());
            then!(before == after)
        }
    }
}
