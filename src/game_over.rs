use state::State;
use state::StateChange;
use game::History;
use rest;
use score::Score;
use score::ScoreMessage;

use regex::Regex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub struct GameOver {
    pub hiscores: Option<HighScores>,
    pub score: Score,
    history: History,
}

pub struct HighScores {
    pub higher_scores: Vec<Score>,
    pub lower_scores: Vec<Score>,
    pub has_hiscore: bool,
}

impl HighScores {
    fn new(hiscores: &[Score], user_score: &Score) -> Self {
        let index = match hiscores.binary_search(user_score) {
            Ok(i) | Err(i) => i,
        };

        let (higher_scores, lower_scores) = hiscores.split_at(index);

        let mut lower_scores = lower_scores.to_vec();
        let displaced_score = lower_scores.pop();

        let has_hiscore = higher_scores.is_empty() || displaced_score.is_some();

        HighScores {
            higher_scores: higher_scores.to_vec(),
            lower_scores,
            has_hiscore,
        }
    }

    fn has_hiscore(&self) -> bool {
        self.has_hiscore
    }
}

impl GameOver {
    pub fn new(score: u32, history: History) -> Self {
        let hiscores = rest::get_hiscores();

        if let Err(ref e) = hiscores {
            println!("Failed to retrieve hiscores: {}", e);
        }

        let score = Score::new(score, "".to_string());

        let hiscores = hiscores.ok().map(|h| HighScores::new(&h, &score));

        GameOver {
            hiscores,
            score,
            history,
        }
    }

    pub fn posting_hiscore(&self) -> bool {
        self.hiscores.as_ref().map_or(
            false,
            HighScores::has_hiscore,
        )
    }

    pub fn update(&mut self, events: &[Event]) -> StateChange {
        lazy_static! {
            static ref ALPHANUMERIC: Regex = Regex::new("^[a-zA-Z0-9]$").unwrap();
        }

        for event in events {
            match *event {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Return => {
                            if !self.posting_hiscore() || !self.score.name.is_empty() {
                                if self.posting_hiscore() {
                                    let message =
                                        ScoreMessage::new(self.score.clone(), self.history.clone());
                                    rest::post_hiscore(&message);
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
                    if !self.posting_hiscore() {
                        return StateChange::Replace(State::play());
                    }
                }
                _ => {}
            }
        }

        StateChange::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_there_are_no_high_scores_then_this_is_a_new_highscore() {
        let high_scores = HighScores::new(&vec![], &Score::new(100, "AEL".to_owned()));

        assert!(high_scores.has_hiscore());
    }

    #[test]
    fn when_there_is_a_lower_highscore_then_this_is_a_new_highscore() {
        let high_scores = HighScores::new(
            &vec![
                Score::new(1000, "ALC".to_owned()),
                Score::new(500, "BOB".to_owned()),
                Score::new(400, "CHR".to_owned()),
            ],
            &Score::new(750, "AEL".to_owned()),
        );

        println!("{:?}", high_scores.higher_scores);
        println!("{:?}", high_scores.lower_scores);

        assert!(high_scores.has_hiscore());
    }

    #[test]
    fn when_there_is_a_lower_highscore_then_the_lowest_score_is_removed() {
        let high_scores = HighScores::new(
            &vec![
                Score::new(1000, "ALC".to_owned()),
                Score::new(500, "BOB".to_owned()),
                Score::new(400, "CHR".to_owned()),
            ],
            &Score::new(750, "AEL".to_owned()),
        );

        assert_eq!(
            high_scores.lower_scores,
            vec![Score::new(500, "BOB".to_owned())]
        );
    }

    #[test]
    fn when_this_is_the_highest_score_then_there_is_a_new_highscore() {
        let high_scores = HighScores::new(
            &vec![
                Score::new(1000, "ALC".to_owned()),
                Score::new(500, "BOB".to_owned()),
            ],
            &Score::new(2000, "AEL".to_owned()),
        );

        assert!(high_scores.has_hiscore());
    }

    #[test]
    fn when_all_high_scores_are_larger_then_this_is_not_a_highscore() {
        let high_scores = HighScores::new(
            &vec![
                Score::new(1000, "ALC".to_owned()),
                Score::new(500, "BOB".to_owned()),
            ],
            &Score::new(100, "AEL".to_owned()),
        );

        assert!(!high_scores.has_hiscore());
    }
}
