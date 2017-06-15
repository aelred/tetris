use game::GamePlay;
use draw::Drawer;
use game_over::GameOver;
use rest::Client;

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

    pub fn update(
        &mut self,
        drawer: &mut Drawer,
        events: &[Event],
        client: &mut Client,
    ) -> StateChange {
        match *self {
            State::Title => State::title_update(drawer, events),
            State::Play(ref mut game) => game.update(drawer, events, client),
            State::Paused => State::pause_update(drawer, events),
            State::GameOver(ref mut game_over) => game_over.update(drawer, events, client),
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

        drawer
            .text()
            .size(4)
            .centered()
            .draw("Tetris")
            .size(1)
            .under()
            .offset(0, 10)
            .draw("[ Press Enter ]");

        StateChange::None
    }

    fn pause_update(drawer: &mut Drawer, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::Window { win_event: FocusGained, .. } = *event {
                return StateChange::Pop;
            }
        }

        drawer.text().centered().draw("Paused");

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
