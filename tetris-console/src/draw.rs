use std::io;
use std::io::Result;
use std::io::Write;

use termion::color;
use termion::cursor;

use tetris::Board;
use tetris::Game;
use tetris::Piece;
use tetris::ShapeColor;
use tetris::State;

const TITLE: &str = r#"
╔════════════════════╗
║                    ║
║                    ║
║                    ║
║    ===========     ║
║    T E T R I S     ║
║    ===========     ║
║                    ║
║                    ║
║                    ║
║                    ║
║       PRESS        ║
║       ENTER        ║
║                    ║
║                    ║
║                    ║
║                    ║
║                    ║
║                    ║
║                    ║
║                    ║
╚════════════════════╝
"#;

const BLOCK_WIDTH: u16 = 2;
const BLOCK: &str = "▐▉";

const HOR_BORDER: &str = "══";
const VERT_BORDER: &str = "║";
const TL_BORDER: &str = "╔";
const BL_BORDER: &str = "╚";
const TR_BORDER: &str = "╗";
const BR_BORDER: &str = "╝";

pub fn draw<W: Write>(stdout: &mut W, state: &mut State) -> Result<()> {
    let mut buffer = io::BufWriter::new(stdout);

    match &state {
        State::Title(_) => {
            draw_title(&mut buffer)?;
        }
        State::Play(ref game) => {
            draw_game(&mut buffer, &game.game)?;
        }
        State::Paused(_) => {
            // TODO
            write!(buffer, "{}PAUSED", cursor::Goto(1, 1))?;
        }
        State::GameOver(_) => {
            // TODO
            write!(buffer, "{}Not implemented", cursor::Goto(1, 1))?;
        }
    }

    buffer.flush()
}

fn draw_title<W: Write>(stdout: &mut W) -> Result<()> {
    for (row, line) in TITLE.lines().enumerate() {
        write!(stdout, "{}{}", cursor::Goto(1, row as u16), line)?;
    }
    Ok(())
}

fn draw_game<W: Write>(stdout: &mut W, game: &Game) -> Result<()> {
    draw_border(stdout)?;
    draw_board(stdout, &game.board)?;
    draw_piece(stdout, &game.piece)
}

fn draw_board<W: Write>(stdout: &mut W, board: &Board) -> Result<()> {
    for (num, row) in board.grid[Board::HIDE_ROWS as usize..].iter().enumerate() {
        write!(stdout, "{}", cursor::Goto(2, num as u16 + 2))?;

        for cell in row {
            match cell {
                Some(shape_color) => {
                    set_shape_color(stdout, *shape_color)?;
                    write!(stdout, "{}", BLOCK)?;
                }
                None => {
                    write!(stdout, "  ")?;
                }
            };
        }
    }

    Ok(())
}

fn draw_border<W: Write>(stdout: &mut W) -> Result<()> {
    write!(stdout, "{}", color::Fg(color::White))?;

    let hor_border = HOR_BORDER.repeat(Board::WIDTH as usize);

    write!(
        stdout,
        "{}{}{}{}",
        cursor::Goto(1, 1),
        TL_BORDER,
        hor_border,
        TR_BORDER
    )?;

    const RIGHT_BORDER_COLUMN: u16 = (Board::WIDTH as u16 * BLOCK_WIDTH) + 2;

    for row in 0..u16::from(Board::VISIBLE_ROWS) {
        write!(stdout, "{}{}", cursor::Goto(1, row + 2), VERT_BORDER)?;
        write!(
            stdout,
            "{}{}",
            cursor::Goto(RIGHT_BORDER_COLUMN, row + 2),
            VERT_BORDER
        )?;
    }

    const BOTTOM_ROW: cursor::Goto = cursor::Goto(1, Board::VISIBLE_ROWS as u16 + 2);

    write!(
        stdout,
        "{}{}{}{}",
        BOTTOM_ROW, BL_BORDER, hor_border, BR_BORDER
    )
}

fn draw_piece<W: Write>(stdout: &mut W, piece: &Piece) -> Result<()> {
    set_shape_color(stdout, piece.shape.color)?;

    for pos in piece.blocks() {
        if pos.y() >= i16::from(Board::HIDE_ROWS) {
            let cursor_x = (pos.x() as u16) * BLOCK_WIDTH + 2;
            let cursor_y = (pos.y() - i16::from(Board::HIDE_ROWS) + 2) as u16;
            let cursor = cursor::Goto(cursor_x, cursor_y);
            write!(stdout, "{}{}", cursor, BLOCK)?;
        }
    }

    Ok(())
}

fn set_shape_color<W: Write>(stdout: &mut W, shape_color: ShapeColor) -> Result<()> {
    match shape_color {
        ShapeColor::O => write!(stdout, "{}", color::Fg(color::Yellow)),
        ShapeColor::I => write!(stdout, "{}", color::Fg(color::Cyan)),
        ShapeColor::J => write!(stdout, "{}", color::Fg(color::Blue)),
        ShapeColor::L => write!(stdout, "{}", color::Fg(color::White)),
        ShapeColor::S => write!(stdout, "{}", color::Fg(color::Green)),
        ShapeColor::T => write!(stdout, "{}", color::Fg(color::Magenta)),
        ShapeColor::Z => write!(stdout, "{}", color::Fg(color::Red)),
    }
}
