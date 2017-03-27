use game::Game;
use draw::Drawer;

use sdl2::event::Event;
use sdl2::event::WindowEvent::FocusGained;
use sdl2::keyboard::Keycode;

pub enum State {
    Title,
    Play(Box<Game>),
    Paused,
    GameOver { score: u32 },
}

impl State {
    fn play() -> State {
        State::Play(Box::new(Game::new()))
    }

    pub fn update(&mut self, drawer: &mut Drawer, events: &[Event]) -> StateChange {
        match *self {
            State::Title => State::title_update(drawer, events),
            State::Play(ref mut game) => game.update(drawer, events),
            State::Paused => State::pause_update(drawer, events),
            State::GameOver { score } => State::game_over_update(score, drawer, events),
        }
    }

    fn title_update(drawer: &mut Drawer, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::KeyDown { keycode: Some(keycode), .. } = *event {
                if let Keycode::Return = keycode {
                    return StateChange::Push(State::play());
                }
            }
        }

        let tetris_target = drawer.draw_text_centered("Tetris", 0, 0, 4);

        drawer.draw_text_centered("[ Press Enter ]", 0, tetris_target.height() as i32, 1);


        StateChange::None
    }

    fn pause_update(drawer: &mut Drawer, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::Window { win_event: FocusGained, .. } = *event {
                return StateChange::Pop;
            }
        }

        drawer.draw_text_centered("Paused", 0, 0, 1);

        StateChange::None
    }

    fn game_over_update(score: u32, drawer: &mut Drawer, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::KeyDown { keycode: Some(keycode), .. } = *event {
                if let Keycode::Return = keycode {
                    return StateChange::Replace(State::play());
                }
            }
        }

        let game_over_target = drawer.draw_text_centered("Game Over", 0, 0, 3);
        let score_header = drawer.draw_text_under("final score", &game_over_target, 10, 1);
        let score_text = drawer.draw_text_under(&score.to_string(), &score_header, 0, 3);

        drawer.draw_text_under("[ Press Enter ]", &score_text, 10, 1);

        StateChange::None
    }
}

pub enum StateChange {
    None,
    Push(State),
    Pop,
    Replace(State),
}

impl StateChange {
    pub fn apply(self, states: &mut Vec<State>) {
        match self {
            StateChange::None => {}
            StateChange::Push(state) => {
                states.push(state);
            }
            StateChange::Pop => {
                states.pop();
            }
            StateChange::Replace(state) => {
                states.pop();
                states.push(state);
            }
        }
    }
}
