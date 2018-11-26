extern crate termion;
extern crate tetris;

use std::io;
use std::io::Write;
use std::io::Result;
use termion::color;
use termion::color::Rgb;
use termion::cursor;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tetris::shape::ShapeColor;
use tetris::state::State;
use termion::event::Key;
use std::time::Duration;
use tetris::piece::Piece;
use tetris::board;
use tetris::game::GameWithHistory;
use tetris::game::Game;
use tetris::board::Board;
use std::fmt::Display;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;
    let mut stdin = termion::async_stdin().keys();

    let mut state = State::default();

    write!(stdout, "{}{}", cursor::Hide, termion::clear::All)?;

    loop {
        draw(&mut stdout, &mut state)?;

        std::thread::sleep(Duration::from_millis(33));

        if let Some(key) = stdin.next() {
            let key = key?;

            if let Key::Char('q') = key {
                write!(
                    stdout,
                    "{}{}{}{}{}",
                    termion::clear::All,
                    color::Fg(color::Reset),
                    color::Bg(color::Reset),
                    cursor::Goto(1, 1),
                    cursor::Show
                )?;
                break;
            }

            state = match state {
                State::Title(title) => match key {
                    Key::Char('\n') => title.start_game(),
                    _ => State::from(title),
                },
                State::Play(mut game) => handle_key_in_game(game, key),
                State::Paused(paused) => paused.unpause(),
                State::GameOver(game_over) => State::from(game_over),
            };
        }

        state = state.update();
    }

    Ok(())
}

fn handle_key_in_game(mut game: GameWithHistory, key: Key) -> State {
    match key {
        Key::Up => game.rotate(),
        Key::Left => game.move_left(),
        Key::Right => game.move_right(),
        Key::Down => game.start_soft_drop(),
        Key::Char(' ') => game.start_hard_drop(),
        Key::Char('\n') => return game.pause(),
        _ => {}
    };

    State::from(game)
}

const BLOCK_WIDTH: u16 = 2;
const BLOCK: &str = "▐▉";

const HOR_BORDER: &str = "══";
const VERT_BORDER: &str = "║";
const TL_BORDER: &str = "╔";
const BL_BORDER: &str = "╚";
const TR_BORDER: &str = "╗";
const BR_BORDER: &str = "╝";

fn draw<W: Write>(stdout: &mut W, state: &mut State) -> Result<()> {
    let mut buffer = io::BufWriter::new(stdout);

    match &state {
        State::Title(_) => {
            write!(buffer, "{}TETRIS: Press Enter", cursor::Goto(1, 1))?;
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

fn draw_game<W: Write>(stdout: &mut W, game: &Game) -> Result<()> {
    draw_border(stdout)?;
    draw_board(stdout, &game.board)?;
    draw_piece(stdout, &game.piece)
}

fn draw_board<W: Write>(stdout: &mut W, board: &Board) -> Result<()> {
    for (num, row) in board.grid.iter().enumerate() {
        write!(stdout, "{}", cursor::Goto(2, num as u16 + 2))?;

        for cell in row.iter() {
            match cell {
                Some(shape_color) => {
                    write!(stdout, "{}{}", shape_color_to_ascii_color(*shape_color), BLOCK)?;
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

    let hor_border = HOR_BORDER.repeat(board::WIDTH as usize);

    write!(stdout, "{}{}{}{}", cursor::Goto(1, 1), TL_BORDER, hor_border, TR_BORDER)?;

    const RIGHT_BORDER_COLUMN: u16 = (board::WIDTH as u16 * BLOCK_WIDTH) + 2;

    for row in 0..u16::from(board::HEIGHT) {
        write!(stdout, "{}{}", cursor::Goto(1, row + 2), VERT_BORDER)?;
        write!(stdout, "{}{}", cursor::Goto(RIGHT_BORDER_COLUMN, row + 2), VERT_BORDER)?;
    }

    const BOTTOM_ROW: cursor::Goto = cursor::Goto(1, board::HEIGHT as u16 + 2);

    write!(stdout, "{}{}{}{}", BOTTOM_ROW, BL_BORDER, hor_border, BR_BORDER)
}

fn draw_piece<W: Write>(stdout: &mut W, piece: &Piece) -> Result<()> {
    write!(stdout, "{}", shape_color_to_ascii_color(piece.shape.color))?;

    for pos in piece.blocks().iter() {
        let cursor_x = (pos.x() as u16) * BLOCK_WIDTH + 2;
        let cursor_y = pos.y() as u16 + 2;
        let cursor = cursor::Goto(cursor_x, cursor_y);
        write!(stdout, "{}{}", cursor, BLOCK)?;
    }

    Ok(())
}

fn shape_color_to_ascii_color(shape_color: ShapeColor) -> impl Display {
    let color = match shape_color {
        ShapeColor::O => Rgb(255, 255, 0),
        ShapeColor::I => Rgb(0, 255, 255),
        ShapeColor::J => Rgb(0, 0, 255),
        ShapeColor::L => Rgb(255, 165, 0),
        ShapeColor::S => Rgb(0, 255, 0),
        ShapeColor::T => Rgb(255, 0, 255),
        ShapeColor::Z => Rgb(255, 0, 0),
    };

    color::Fg(color)
}
