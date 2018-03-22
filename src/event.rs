use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::event::WindowEvent;
use sdl2::EventPump;
use draw::WINDOW_RATIO;
use game::Action;
use std;
use state::State;
use state::StateChange;
use game::GamePlay;
use game_over::GameOver;
use score::ScoreMessage;
use rest;
use regex::Regex;

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

    pub fn events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().collect()
    }

    pub fn actions(&mut self) -> Vec<Action> {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();
        events
            .iter()
            .flat_map(|event| self.event_to_action(event))
            .collect()
    }

    pub fn handle(&mut self, state: &mut State) -> StateChange {
        match *state {
            State::Title => self.handle_title(),
            State::Play(ref mut game) => self.handle_game(game),
            State::Paused => self.handle_paused(),
            State::GameOver(ref mut game_over) => self.handle_game_over(game_over),
        }
    }

    fn handle_title(&mut self) -> StateChange {
        for event in self.events() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Return), .. } |
                Event::FingerUp { .. } => {
                    return StateChange::Push(State::play());
                }
                _ => {}
            }
        }

        StateChange::None
    }

    fn handle_game(&mut self, game: &mut GamePlay) -> StateChange {
        for action in self.actions() {
            if let Some(state_change) = game.apply_action(action) {
                return state_change;
            }
        }

        StateChange::None
    }

    fn handle_paused(&mut self) -> StateChange {
        for event in self.events() {
            if let Event::Window { win_event: WindowEvent::FocusGained, .. } = event {
                return StateChange::Pop;
            }
        }

        StateChange::None
    }

    fn handle_game_over(&mut self, game_over: &mut GameOver) -> StateChange {
        lazy_static! {
            static ref ALPHANUMERIC: Regex = Regex::new("^[a-zA-Z0-9]$").unwrap();
        }

        for event in self.events() {
            match event {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Return => {
                            if !game_over.posting_hiscore() || !game_over.score.name.is_empty() {
                                if game_over.posting_hiscore() {
                                    let message = ScoreMessage::new(
                                        game_over.score.clone(),
                                        game_over.history.clone(),
                                    );
                                    rest::post_hiscore(&message);
                                }
                                return StateChange::Replace(State::play());
                            }
                        }
                        Keycode::Backspace => {
                            game_over.score.name.pop();
                        }
                        k if ALPHANUMERIC.is_match(&k.name()) => {
                            game_over.score.name.push_str(&k.name());
                            game_over.score.name.truncate(3);
                        }
                        _ => {}
                    }
                }
                Event::FingerUp { .. } => {
                    // TODO: Find a way to submit high-scores with touch
                    if !game_over.posting_hiscore() {
                        return StateChange::Replace(State::play());
                    }
                }
                _ => {}
            }
        }

        StateChange::None
    }

    fn event_to_action(&mut self, event: &Event) -> Option<Action> {
        match *event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => exit(),
            Event::Window { win_event: WindowEvent::FocusLost, .. } => Some(Action::Pause),
            Event::KeyDown { keycode: Some(keycode), .. } => {
                match keycode {
                    Keycode::Left => Some(Action::MoveLeft),
                    Keycode::Right => Some(Action::MoveRight),
                    Keycode::Up => Some(Action::Rotate),
                    Keycode::Down => Some(Action::StartSoftDrop),
                    Keycode::Space => Some(Action::StartHardDrop),
                    _ => None,
                }
            }
            Event::KeyUp { keycode: Some(Keycode::Down), .. } => Some(Action::StopDrop),
            Event::FingerDown { x, y, timestamp, .. } => {
                self.last_finger_press = Some(FingerPress { x, y, timestamp });
                None
            }
            Event::FingerUp { x, y, timestamp, .. } => {
                self.last_finger_press.take().map(|last_finger_press| {
                    let finger_press = FingerPress { x, y, timestamp };
                    let (vx, vy) = finger_press.velocity(&last_finger_press);
                    if vx < -FINGER_SENSITIVITY {
                        Action::MoveLeft
                    } else if vx > FINGER_SENSITIVITY {
                        Action::MoveRight
                    } else if vy > FINGER_SENSITIVITY {
                        Action::StartHardDrop
                    } else {
                        Action::Rotate
                    }
                })
            }
            _ => None,
        }
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
