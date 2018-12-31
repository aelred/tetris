use crate::game::Game;
use crate::game_over::GameOver;

/// The state of the entire Tetris application.
pub enum State {
    /// The title screen.
    Title(Title),
    /// The in-game screen.
    Play(Game),
    /// The paused screen.
    Paused(Paused),
    /// The game over screen.
    GameOver(GameOver),
}

impl State {
    /// Create a title screen state.
    pub fn title() -> State {
        State::Title(Title)
    }

    /// Create a game-play state.
    pub fn play() -> State {
        State::Play(Game::default())
    }

    /// Create a paused state for the given game.
    pub fn paused(game: Game) -> State {
        State::Paused(Paused(game))
    }

    /// Update the given state, ticking time forward once.
    pub fn update(self) -> Self {
        match self {
            State::Play(game) => game.update(),
            _ => self,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::title()
    }
}

/// The title screen state.
pub struct Title;

impl Title {
    /// Start a game from the title screen.
    pub fn start_game(self) -> State {
        State::play()
    }
}

/// The paused state for the underlying game.
pub struct Paused(pub Game);

impl Paused {
    /// Unpause the game.
    pub fn unpause(self) -> State {
        State::Play(self.0)
    }
}
