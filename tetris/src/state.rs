use game::GamePlay;
use game_over::GameOver;
use std;

pub enum State {
    Title(Title),
    Play(GamePlay),
    Paused(Paused),
    GameOver(GameOver),
}

impl State {
    pub fn title() -> State {
        State::from(Title)
    }

    pub fn play() -> State {
        State::from(GamePlay::default())
    }

    pub fn paused(game_play: GamePlay) -> State {
        State::from(Paused(game_play))
    }

    pub fn update(&mut self) {
        if let &mut State::Play(game) = self {
            std::mem::replace(self, game.update());
        }
    }
}

impl From<Title> for State {
    fn from(title: Title) -> Self {
        State::Title(title)
    }
}

impl From<GamePlay> for State {
    fn from(game_play: GamePlay) -> Self {
        State::Play(game_play)
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

pub struct Paused(pub GamePlay);

impl Paused {
    pub fn unpause(self) -> State {
        State::from(self.0)
    }
}
