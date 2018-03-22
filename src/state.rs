use game::GamePlay;
use game_over::GameOver;

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

    pub fn update(&mut self) -> StateChange {
        match *self {
            State::Play(ref mut game) => game.update(),
            _ => StateChange::None,
        }
    }

    pub fn unpause() -> StateChange {
        StateChange::Pop
    }

    pub fn start_game() -> StateChange {
        StateChange::Push(State::play())
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
