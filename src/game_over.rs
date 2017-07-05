use draw::TextDrawer;
use draw::Drawer;
use state::State;
use state::StateChange;
use score::OFFSET;
use game::History;
use rest::Client;
use score::Score;
use score::ScoreMessage;

use regex::Regex;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub struct GameOver {
    hiscores: Option<HighScores>,
    score: Score,
    history: History,
    posting_hiscore: bool,
}

struct HighScores {
    higher_scores: Vec<Score>,
    lower_scores: Vec<Score>,
}

impl HighScores {
    fn new(hiscores: Vec<Score>, user_score: &Score) -> Self {
        let index = match hiscores.binary_search(user_score) {
            Ok(i) | Err(i) => i,
        };

        let (higher_scores, lower_scores) = hiscores.split_at(index);

        let mut lower_scores = lower_scores.to_vec();
        lower_scores.pop();

        HighScores {
            higher_scores: higher_scores.to_vec(),
            lower_scores: lower_scores,
        }
    }

    fn has_hiscore(&self) -> bool {
        self.lower_scores.is_empty()
    }
}

impl GameOver {
    pub fn new(score: u32, history: History, client: &mut Client) -> Self {
        let hiscores = client.get_hiscores();

        if let Err(ref e) = hiscores {
            println!("Failed to retrieve hiscores: {}", e);
        }

        let score = Score::new(score, "".to_string());

        let hiscores = hiscores.ok().map(|h| HighScores::new(h, &score));

        let posting_hiscore = match hiscores {
            Some(ref hiscores) => hiscores.has_hiscore(),
            None => false,
        };

        GameOver {
            hiscores: hiscores,
            score: score,
            history: history,
            posting_hiscore: posting_hiscore,
        }
    }

    pub fn update(
        &mut self,
        drawer: &mut Drawer,
        events: &[Event],
        client: &mut Client,
    ) -> StateChange {

        lazy_static! {
            static ref ALPHANUMERIC: Regex = Regex::new("^[a-zA-Z0-9]$").unwrap();
        }

        for event in events {
            match *event {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Return => {
                            if !self.posting_hiscore || !self.score.name.is_empty() {
                                if self.posting_hiscore {
                                    let message =
                                        ScoreMessage::new(self.score.clone(), self.history.clone());
                                    client.post_hiscore(&message);
                                }
                                return StateChange::Replace(State::play());
                            }
                        }
                        Keycode::Backspace => {
                            self.score.name.pop();
                        }
                        k if ALPHANUMERIC.is_match(&k.name()) => {
                            self.score.name.push_str(&k.name());
                            self.score.name.truncate(3);
                        }
                        _ => {}
                    }
                }
                Event::FingerUp { .. } => {
                    // TODO: Find a way to submit high-scores with touch
                    if !self.posting_hiscore {
                        return StateChange::Replace(State::play());
                    }
                }
                _ => {}
            }
        }

        let mut text = drawer
            .text()
            .top()
            .offset(0, 50)
            .size(3)
            .draw("Game Over")
            .under()
            .offset(0, 10)
            .size(1)
            .draw("final score")
            .under()
            .size(3)
            .draw(&self.score.value.to_string());

        text = self.draw_hiscores(text);

        if self.posting_hiscore {
            text.size(1).draw("[ Enter Name and Press Enter ]");
        } else {
            text.size(1).draw("[ Press Enter ]");
        }

        StateChange::None
    }

    fn draw_hiscores<'a, 'b>(&self, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b> {

        match self.hiscores {
            Some(HighScores {
                     ref higher_scores,
                     ref lower_scores,
                 }) => {
                let mut text = text.size(3).under().offset(0, 10).draw("High Scores");

                text = text.size(2).under().offset(0, 10);

                for score in higher_scores {
                    text = score.draw(text);
                }

                if self.posting_hiscore {
                    text = self.score
                        .draw(text.color(Color::RGB(255, 255, 100)))
                        .reset_color();
                }

                for score in lower_scores {
                    text = score.draw(text);
                }

                text.under().offset(-OFFSET, 10)
            }
            None => {
                text.size(1)
                    .under()
                    .offset(0, 10)
                    .draw("[ ERROR Failed to retrieve High Scores ]")
                    .offset(0, 20)
            }
        }
    }
}
