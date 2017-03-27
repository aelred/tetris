use game::Game;
use draw::draw_text_centered;
use draw::draw_text_under;

use sdl2::render::Renderer;
use sdl2::ttf::Font;
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

    pub fn update(&mut self,
                  renderer: &mut Renderer,
                  font: &Font,
                  events: &[Event])
                  -> StateChange {
        match *self {
            State::Title => State::title_update(renderer, font, events),
            State::Play(ref mut game) => game.update(renderer, font, events),
            State::Paused => State::pause_update(renderer, font, events),
            State::GameOver { score } => State::game_over_update(score, renderer, font, events),
        }
    }

    fn title_update(renderer: &mut Renderer, font: &Font, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::KeyDown { keycode: Some(keycode), .. } = *event {
                if let Keycode::Return = keycode {
                    return StateChange::Push(State::play());
                }
            }
        }

        let tetris_target = draw_text_centered("Tetris", 0, 0, 4, renderer, font);

        draw_text_centered("[ Press Enter ]",
                           0,
                           tetris_target.height() as i32,
                           1,
                           renderer,
                           font);


        StateChange::None
    }

    fn pause_update(renderer: &mut Renderer, font: &Font, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::Window { win_event: FocusGained, .. } = *event {
                return StateChange::Pop;
            }
        }

        renderer.set_viewport(None);

        draw_text_centered("Paused", 0, 0, 1, renderer, font);

        StateChange::None
    }

    fn game_over_update(score: u32,
                        renderer: &mut Renderer,
                        font: &Font,
                        events: &[Event])
                        -> StateChange {
        for event in events {
            if let Event::KeyDown { keycode: Some(keycode), .. } = *event {
                if let Keycode::Return = keycode {
                    return StateChange::Replace(State::play());
                }
            }
        }

        let game_over_target = draw_text_centered("Game Over", 0, 0, 3, renderer, font);
        let score_header = draw_text_under("final score", &game_over_target, 10, 1, renderer, font);
        let score_text = draw_text_under(&score.to_string(), &score_header, 0, 3, renderer, font);

        draw_text_under("[ Press Enter ]", &score_text, 10, 1, renderer, font);

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
