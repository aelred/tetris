use game::GamePlay;
use game_over::GameOver;

use sdl2::event::Event;
use sdl2::event::WindowEvent::FocusGained;
use sdl2::keyboard::Keycode;

pub enum State {
    Title,
    Play(Box<GamePlay>),
    Paused,
    GameOver(GameOver),
}

impl State {
    pub fn play() -> State {
        State::Play(Box::new(GamePlay::default()))
    }

    pub fn update(&mut self, events: &[Event]) -> StateChange {
        match *self {
            State::Title => State::title_update(events),
            State::Play(ref mut game) => game.update(events),
            State::Paused => State::pause_update(events),
            State::GameOver(ref mut game_over) => game_over.update(events),
        }
    }

    fn title_update(events: &[Event]) -> StateChange {
        for event in events {
            match *event {
                Event::KeyDown { keycode: Some(Keycode::Return), .. } |
                Event::FingerUp { .. } => {
                    return StateChange::Push(State::play());
                }
                _ => {}
            }
        }

        StateChange::None
    }

    fn pause_update(events: &[Event]) -> StateChange {
        for event in events {
            if let Event::Window { win_event: FocusGained, .. } = *event {
                return StateChange::Pop;
            }
        }

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
