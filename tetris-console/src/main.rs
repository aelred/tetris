use termion;


use std::io;
use std::io::Result;
use std::io::Write;
use std::time::Duration;

use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use tetris::game::GameWithHistory;
use tetris::state::State;

mod draw;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;
    let mut stdin = termion::async_stdin().keys();

    let mut state = State::default();

    write!(stdout, "{}{}", cursor::Hide, termion::clear::All)?;

    loop {
        draw::draw(&mut stdout, &mut state)?;

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
                State::Play(game) => handle_key_in_game(game, key),
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
