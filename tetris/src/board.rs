use crate::game::StepResult;
use crate::piece::Piece;
use crate::pos::Pos;
use crate::shape::ShapeColor;

/// The board state, describing which cells are full and what colour tetromino they were filled
/// with.
#[derive(Clone, Debug, Default)]
pub struct Board {
    grid: [[Option<ShapeColor>; Board::WIDTH as usize]; Board::HEIGHT as usize],
}

impl Board {
    /// Width of the playable board in cells.
    pub const WIDTH: u8 = 10;

    /// Height of the playable board in cells - note that some of the top-most cells are not visible
    /// in play, indicated by `HIDE_ROWS`.
    pub const HEIGHT: u8 = 24;

    /// Number of rows at the top of the board that are not visible. This is where new pieces are
    /// spawned.
    pub const HIDE_ROWS: u8 = 4;

    /// Number of visible rows, based on total height and number of hidden rows.
    pub const VISIBLE_ROWS: u8 = Board::HEIGHT - Board::HIDE_ROWS;

    /// Get the grid of cells of fixed tetrominos.
    pub fn grid(&self) -> &[[Option<ShapeColor>; Board::WIDTH as usize]; Board::HEIGHT as usize] {
        &self.grid
    }

    /// Get the visible grid of cells of fixed tetrominos, excluding any rows above the visible
    /// portion of the board.
    pub fn visible_grid(&self) -> &[[Option<ShapeColor>; Board::WIDTH as usize]] {
        &self.grid[Board::HIDE_ROWS as usize..]
    }

    /// Lock a piece, attaching it to the board permanently and potentially clearing some rows.
    ///
    /// This can cause a game over if the piece is locked above the visible playing area, which
    /// will be indicated in the return value.
    pub fn lock_piece(&mut self, piece: &Piece) -> FillResult {
        let mut step_result = StepResult::GameOver;

        for cell in piece.blocks() {
            if cell.y() > i16::from(Board::HIDE_ROWS) {
                step_result = StepResult::Continue;
            }
            self.fill_pos(cell, piece.shape.color);
        }

        FillResult {
            step_result,
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

        for y in 0..Board::HEIGHT {
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

        for x in 0..Board::WIDTH as usize {
            self.grid[0][x] = None;
        }
    }

    /// Returns if this position on the board is free and in-bounds
    pub fn is_pos_free(&self, pos: Pos) -> bool {
        !out_bounds(pos) && self.grid[pos.y() as usize][pos.x() as usize].is_none()
    }
}

/// The result of calling [`Board::lock_piece`](struct.Board.html#method.lock_piece), which is
/// called when locking a piece. Indicates if this caused a game over, or if any lines were cleared.
pub struct FillResult {
    pub step_result: StepResult,
    pub lines_cleared: u32,
}

/// Return whether the given position is out of bounds of the board (including hidden rows).
fn out_bounds(pos: Pos) -> bool {
    pos.x() < 0
        || pos.y() < 0
        || pos.x() >= i16::from(Board::WIDTH)
        || pos.y() >= i16::from(Board::HEIGHT)
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use quickcheck::{quickcheck, Arbitrary, Gen, TestResult};

    use super::*;

    impl Arbitrary for Board {
        fn arbitrary<G: Gen>(g: &mut G) -> Board {
            unsafe {
                let mut array: [[Option<ShapeColor>; Board::WIDTH as usize];
                    Board::HEIGHT as usize] =
                    [[None; Board::WIDTH as usize]; Board::HEIGHT as usize];

                for row in &mut array {
                    for cell in row {
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
                g.gen_range(0, Board::WIDTH as i16),
                g.gen_range(0, Board::HEIGHT as i16),
            ))
        }
    }

    quickcheck! {

        fn a_new_board_is_empty(pos: InBoundsPos) -> bool {
            Board::default().is_pos_free(pos.0)
        }

        fn after_filling_a_space_it_is_filled(
            board: Board, pos: InBoundsPos, col: ShapeColor) -> bool {
            let pos = pos.0;
            let mut board = board;
            board.fill_pos(pos, col);
            !board.is_pos_free(pos)
        }

        fn after_filling_a_space_no_other_space_changes(
            board: Board, pos1: InBoundsPos, pos2: Pos, col: ShapeColor) -> TestResult {

            let pos1 = pos1.0;
            let mut board = board;

            when!(pos1 != pos2);

            let free_before = board.is_pos_free(pos2);
            board.fill_pos(pos1, col);
            let free_after = board.is_pos_free(pos2);
            then!(free_before == free_after)
        }

        fn after_clearing_a_row_the_top_row_is_empty(board: Board, pos: InBoundsPos) -> bool {
            let pos = pos.0;
            let mut board = board;
            board.clear_row(pos.y() as u8);
            board.is_pos_free(Pos::new(pos.x(), 0))
        }

        fn after_clearing_a_row_nothing_under_it_is_changed(
            board: Board, y: u8, under: InBoundsPos) -> TestResult {

            let under = under.0;
            let mut board = board;

            when!(under.y() > y as i16);

            let before = board.is_pos_free(under);
            board.clear_row(y);
            let after = board.is_pos_free(under);
            then!(before == after)
        }

        fn after_clearing_a_row_everything_above_it_shifts_down(
            board: Board, y: u8, above: InBoundsPos) -> TestResult {

            let above = above.0;
            let mut board = board;

            when!(y < Board::HEIGHT);
            when!(above.y() < y as i16);

            let before = board.is_pos_free(above);
            board.clear_row(y);
            let after = board.is_pos_free(above.down());
            then!(before == after)
        }
    }
}
