use game::Game;
use game::WINDOW_HEIGHT;
use game::WINDOW_WIDTH;

use sdl2::render::Renderer;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::event::Event;
use sdl2::event::WindowEvent::FocusGained;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub enum State {
    Title,
    Play(Box<Game>),
    Paused,
    GameOver,
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
            State::Play(ref mut game) => game.update(renderer, events),
            State::Paused => State::pause_update(renderer, font, events),
            State::GameOver => State::game_over_update(renderer, font, events),
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

        let tetris_target = draw_text("Tetris", 0, 0, 4, renderer, font);

        draw_text("[ Press Enter ]",
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

        draw_text("Paused", 0, 0, 1, renderer, font);

        StateChange::None
    }

    fn game_over_update(renderer: &mut Renderer, font: &Font, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::KeyDown { keycode: Some(keycode), .. } = *event {
                if let Keycode::Return = keycode {
                    return StateChange::Replace(State::play());
                }
            }
        }

        let game_over_target = draw_text("Game Over", 0, 0, 3, renderer, font);

        draw_text("[ Press Enter ]",
                  0,
                  game_over_target.height() as i32,
                  1,
                  renderer,
                  font);

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

fn draw_text(text: &str,
             offset_x: i32,
             offset_y: i32,
             size: u32,
             renderer: &mut Renderer,
             font: &Font)
             -> Rect {
    let surface = font.render(text).solid(Color::RGB(255, 255, 255)).unwrap();
    let texture = renderer.create_texture_from_surface(&surface).unwrap();

    let TextureQuery { width, height, .. } = texture.query();

    let target = center_view(offset_x, offset_y, width * size, height * size);

    renderer.copy(&texture, None, Some(target)).unwrap();

    target
}

fn center_view(x: i32, y: i32, width: u32, height: u32) -> Rect {
    let center_x = (WINDOW_WIDTH / 2) as i32;
    let center_y = (WINDOW_HEIGHT / 2) as i32;

    Rect::new(center_x + x - width as i32 / 2,
              center_y + y - height as i32 / 2,
              width,
              height)
}
