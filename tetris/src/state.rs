use crate::game::Game;
use crate::game_over::GameOver;

pub enum State {
    Title(Title),
    Play(Game),
    Paused(Paused),
    GameOver(GameOver),
}

impl State {
    pub fn title() -> State {
        State::from(Title)
    }

    pub fn play() -> State {
        State::from(Game::default())
    }

    pub fn paused(game_with_history: Game) -> State {
        State::from(Paused(game_with_history))
    }

    pub fn update(self) -> Self {
        match self {
            State::Play(game) => game.update(),
            _ => self,
        }
    }
}

impl From<Title> for State {
    fn from(title: Title) -> Self {
        State::Title(title)
    }
}

impl From<Game> for State {
    fn from(game_with_history: Game) -> Self {
        State::Play(game_with_history)
    }
}

impl From<Paused> for State {
    fn from(paused: Paused) -> Self {
        State::Paused(paused)
    }
}

impl From<GameOver> for State {
    fn from(game_over: GameOver) -> Self {
        State::GameOver(game_over)
    }
}

impl Default for State {
    fn default() -> Self {
        State::title()
    }
}

pub struct Title;

impl Title {
    pub fn start_game(self) -> State {
        State::play()
    }
}

pub struct Paused(pub Game);

impl Paused {
    pub fn unpause(self) -> State {
        State::from(self.0)
    }
}
