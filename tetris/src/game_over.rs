use std::char;

use crate::game::History;
use crate::rest;
use crate::score::Score;
use crate::score::ScoreMessage;
use crate::state::State;

/// Game over state, where a user can see high-scores and post their high-score.
pub struct GameOver {
    /// Previous high-scores.
    ///
    /// Optional because these are retrieved from the internet, so might not be available.
    pub hiscores: Option<HighScores>,

    /// The user's score.
    pub score: Score,

    /// The history of the game.
    pub history: History,
}

/// High-scores data.
pub struct HighScores {
    /// Scores that are strictly higher than the user's score.
    pub higher_scores: Vec<Score>,

    /// Scores that are lower or equal to the user's score.
    pub lower_scores: Vec<Score>,

    /// States whether the user has a high-score or not for display purposes.
    pub has_hiscore: bool,
}

impl HighScores {
    /// Create by inspecting a list of high-scores and a user's score.
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
    /// Create a new game over state from a user's score and a game history.
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

    /// Return whether the user can post their high-score.
    ///
    /// This is true only if the list of high-scores was retrieved and the user has a high-score.
    pub fn posting_hiscore(&self) -> bool {
        self.hiscores
            .as_ref()
            .map_or(false, HighScores::has_hiscore)
    }

    /// Delete a character from the entered name.
    pub fn backspace(&mut self) {
        self.score.name.pop();
    }

    /// Push some characters to the entered name.
    pub fn push_name(&mut self, str: &str) {
        if str.chars().all(char::is_alphanumeric) {
            self.score.name.push_str(str);
            self.score.name.truncate(3);
        }
    }

    /// Potentially submit a high-score if allowed, then exit the game over state and return a
    /// new game state.
    pub fn submit(self) -> State {
        if !self.posting_hiscore() || !self.score.name.is_empty() {
            if self.posting_hiscore() {
                let message = ScoreMessage::new(self.score.clone(), self.history);
                rest::post_hiscore(&message);
            }
            State::play()
        } else {
            State::GameOver(self)
        }
    }

    /// Exit the game over state and return a new game state.
    pub fn exit(self) -> State {
        State::play()
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
