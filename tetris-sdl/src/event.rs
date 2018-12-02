use std;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use tetris::game::GameWithHistory;
use tetris::game_over::GameOver;
use tetris::state::Paused;
use tetris::state::State;
use tetris::state::Title;

use crate::draw::WINDOW_RATIO;

// the minimum velocity before movement is registered, in % of screen width per ms
const FINGER_SENSITIVITY: f32 = 0.0002;

pub struct EventHandler {
    event_pump: EventPump,
    last_finger_press: Option<FingerPress>,
}

impl EventHandler {
    pub fn new(event_pump: EventPump) -> Self {
        EventHandler {
            event_pump,
            last_finger_press: None,
        }
    }

    fn events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().collect()
    }

    pub fn handle(&mut self, mut state: State) -> State {
        for event in self.events() {
            state = self.handle_event(state, &event);
        }
        state
    }

    fn handle_event(&mut self, state: State, event: &Event) -> State {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => exit(),
            _ => {}
        }

        match state {
            State::Title(title) => self.handle_title(title, event),
            State::Play(game) => self.handle_game(game, event),
            State::Paused(paused) => self.handle_paused(paused, event),
            State::GameOver(game_over) => self.handle_game_over(game_over, event),
        }
    }

    fn handle_title(&mut self, title: Title, event: &Event) -> State {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            }
            | Event::FingerUp { .. } => title.start_game(),
            _ => State::from(title),
        }
    }

    fn handle_game(&mut self, mut game: GameWithHistory, event: &Event) -> State {
        match *event {
            Event::Window {
                win_event: WindowEvent::FocusLost,
                ..
            } => return game.pause(),
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Left => game.move_left(),
                Keycode::Right => game.move_right(),
                Keycode::Up => game.rotate(),
                Keycode::Down => game.start_soft_drop(),
                Keycode::Space => game.start_hard_drop(),
                _ => {}
            },
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => game.stop_drop(),
            Event::FingerDown {
                x, y, timestamp, ..
            } => {
                self.last_finger_press = Some(FingerPress { x, y, timestamp });
            }
            Event::FingerUp {
                x, y, timestamp, ..
            } => {
                if let Some(last_finger_press) = self.last_finger_press.take() {
                    let finger_press = FingerPress { x, y, timestamp };
                    let (vx, vy) = finger_press.velocity(&last_finger_press);
                    if vx < -FINGER_SENSITIVITY {
                        game.move_left();
                    } else if vx > FINGER_SENSITIVITY {
                        game.move_right();
                    } else if vy > FINGER_SENSITIVITY {
                        game.start_hard_drop();
                    } else {
                        game.rotate();
                    }
                }
            }
            _ => {}
        }
        State::from(game)
    }

    fn handle_paused(&mut self, paused: Paused, event: &Event) -> State {
        match event {
            Event::Window {
                win_event: WindowEvent::FocusGained,
                ..
            } => paused.unpause(),
            _ => State::from(paused),
        }
    }

    fn handle_game_over(&mut self, mut game_over: GameOver, event: &Event) -> State {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Return => {
                    return game_over.submit();
                }
                Keycode::Backspace => {
                    game_over.backspace();
                }
                k => {
                    game_over.push_name(&k.name());
                }
            },
            // TODO: Find a way to submit high-scores with touch
            Event::FingerUp { .. } => {
                if !game_over.posting_hiscore() {
                    return game_over.exit();
                }
            }
            _ => {}
        }

        State::from(game_over)
    }
}

fn exit() -> ! {
    std::process::exit(0)
}

pub struct FingerPress {
    pub x: f32,
    pub y: f32,
    pub timestamp: u32,
}

impl FingerPress {
    pub fn velocity(self, other: &FingerPress) -> (f32, f32) {
        let dx = self.x - other.x;
        let dy = (self.y - other.y) * WINDOW_RATIO;
        let dt = (self.timestamp - other.timestamp) as f32;
        (dx / dt, dy / dt)
    }
}
