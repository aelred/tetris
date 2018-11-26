extern crate termion;
extern crate tetris;

use std::io;
use std::io::Write;
use termion::color;
use termion::color::Color;
use termion::color::Rgb;
use termion::cursor;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tetris::shape::ShapeColor;
use tetris::state::State;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    let mut state = State::default();

    write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();

    loop {
        match &state {
            State::Title(_) => {
                write!(stdout, "{}TETRIS: Press Enter", cursor::Goto(1, 1)).unwrap();
            }
            State::Play(ref game) => {
                draw_game(&mut stdout, &game.game);
            }
            State::Paused(_) => {
                // TODO
                write!(stdout, "{}PAUSED", cursor::Goto(1, 1)).unwrap();
            }
            State::GameOver(_) => {
                // TODO
                write!(stdout, "{}Not implemented", cursor::Goto(1, 1)).unwrap();
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(33));

        if let Some(b) = stdin.next().map(Result::unwrap) {
            use termion::event::Key;

            if let Key::Char('q') = b {
                write!(
                    stdout,
                    "{}{}{}{}{}",
                    termion::clear::All,
                    color::Fg(color::Reset),
                    color::Bg(color::Reset),
                    termion::cursor::Goto(1, 1),
                    termion::cursor::Show
                ).unwrap();
                break;
            }

            state = match state {
                State::Title(title) => match b {
                    Key::Char('\n') => title.start_game(),
                    _ => State::from(title),
                },
                State::Play(mut game) => match b {
                    Key::Up => {
                        game.rotate();
                        State::from(game)
                    }
                    Key::Left => {
                        game.move_left();
                        State::from(game)
                    }
                    Key::Right => {
                        game.move_right();
                        State::from(game)
                    }
                    Key::Down => {
                        game.start_soft_drop();
                        State::from(game)
                    }
                    Key::Char(' ') => {
                        game.start_hard_drop();
                        State::from(game)
                    }
                    Key::Char('\n') => game.pause(),
                    _ => State::from(game),
                },
                State::Paused(paused) => paused.unpause(),
                State::GameOver(game_over) => State::from(game_over),
            };
        }

        state = state.update();
    }
}

const BLOCK_WIDTH: u16 = 2;
const SPACE: &str = "  ";
const BLOCK: &str = "▐▉";

const HOR_BORDER: &str = "══";
const VERT_BORDER: &str = "║";
const TL_BORDER: &str = "╔";
const BL_BORDER: &str = "╚";
const TR_BORDER: &str = "╗";
const BR_BORDER: &str = "╝";

fn draw_game<W: Write>(stdout: &mut W, game: &tetris::game::Game) {
    let mut buffer = std::io::BufWriter::new(stdout);
    draw_border(&mut buffer);
    draw_board(&mut buffer, &game.board);
    draw_piece(&mut buffer, &game.piece);
}

fn draw_board<W: Write>(stdout: &mut W, board: &tetris::board::Board) {
    for (num, row) in board.grid.iter().enumerate() {
        write!(stdout, "{}", termion::cursor::Goto(2, num as u16 + 2)).unwrap();

        for cell in row.iter() {
            match cell {
                Some(shape_color) => {
                    write!(
                        stdout,
                        "{}{}",
                        color::Fg(shape_color_to_ascii_color(*shape_color)),
                        BLOCK
                    ).unwrap();
                }
                None => {
                    write!(stdout, "{}{}", color::Fg(color::Reset), SPACE).unwrap();
                }
            };
        }
    }
}

fn draw_border<W: Write>(stdout: &mut W) {
    write!(stdout, "{}", color::Fg(color::White)).unwrap();
    write!(
        stdout,
        "{}{}{}{}",
        termion::cursor::Goto(1, 1),
        TL_BORDER,
        HOR_BORDER.repeat(tetris::board::WIDTH as usize),
        TR_BORDER
    ).unwrap();
    for row in 0..u16::from(tetris::board::HEIGHT) {
        write!(
            stdout,
            "{}{}{}{}",
            termion::cursor::Goto(1, row + 2),
            VERT_BORDER,
            termion::cursor::Goto((u16::from(tetris::board::WIDTH) * BLOCK_WIDTH) + 2, row + 2),
            VERT_BORDER
        ).unwrap();
    }
    write!(
        stdout,
        "{}{}{}{}",
        termion::cursor::Goto(1, u16::from(tetris::board::HEIGHT) + 2),
        BL_BORDER,
        HOR_BORDER.repeat(tetris::board::WIDTH as usize),
        BR_BORDER
    ).unwrap();
}

fn draw_piece<W: Write>(stdout: &mut W, piece: &tetris::piece::Piece) {
    let color = shape_color_to_ascii_color(piece.shape.color);
    write!(stdout, "{}", color::Fg(color)).unwrap();

    for pos in piece.blocks().iter() {
        let cursor_x = (pos.x() as u16) * BLOCK_WIDTH + 2;
        let cursor_y = pos.y() as u16 + 2;
        let cursor = termion::cursor::Goto(cursor_x, cursor_y);
        write!(stdout, "{}{}", cursor, BLOCK).unwrap();
    }
}

fn shape_color_to_ascii_color(shape_color: ShapeColor) -> impl Color {
    match shape_color {
        ShapeColor::O => Rgb(255, 255, 0),
        ShapeColor::I => Rgb(0, 255, 255),
        ShapeColor::J => Rgb(0, 0, 255),
        ShapeColor::L => Rgb(255, 165, 0),
        ShapeColor::S => Rgb(0, 255, 0),
        ShapeColor::T => Rgb(255, 0, 255),
        ShapeColor::Z => Rgb(255, 0, 0),
    }
}
