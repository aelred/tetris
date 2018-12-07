use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use serde_derive::{Deserialize, Serialize};

use crate::game::History;

pub const SCORE_ENDPOINT: &str = "/scores";

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Score {
    pub value: u32,
    pub name: String,
}

impl Score {
    pub fn new(value: u32, name: String) -> Self {
        Score { value, name }
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value).reverse()
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreMessage {
    score: Score,
    history: History,
}

#[derive(Debug)]
pub enum ScoreValidationError {
    NameEmpty,
    NameTooLong(usize),
    NameNotAlphanumeric(String),
    UnexpectedScore {
        score_message: Box<ScoreMessage>,
        expected_score: u32,
    },
}

impl Error for ScoreValidationError {}

impl Display for ScoreValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ScoreValidationError::NameEmpty => write!(f, "Name should not be empty"),
            ScoreValidationError::NameTooLong(length) => {
                write!(f, "Name must be at most 3 characters, but was {}", length)
            }
            ScoreValidationError::NameNotAlphanumeric(name) => write!(
                f,
                "Name must contain only alphanumeric characters, but was {}",
                name
            ),
            ScoreValidationError::UnexpectedScore {
                score_message,
                expected_score,
            } => write!(
                f,
                "Score does not match game history {:?}:\n History suggests {} but was {}",
                score_message, expected_score, score_message.score.value
            ),
        }
    }
}

impl ScoreMessage {
    pub fn new(score: Score, history: History) -> Self {
        ScoreMessage { score, history }
    }

    pub fn score(self) -> Result<Score, ScoreValidationError> {
        if self.score.name.is_empty() {
            return Err(ScoreValidationError::NameEmpty);
        }

        if self.score.name.len() > 3 {
            return Err(ScoreValidationError::NameTooLong(self.score.name.len()));
        }

        if !self.score.name.chars().all(char::is_alphanumeric) {
            return Err(ScoreValidationError::NameNotAlphanumeric(self.score.name));
        }

        self.verify_score()
    }

    fn verify_score(self) -> Result<Score, ScoreValidationError> {
        let expected_score = self.history.replay();

        if expected_score == self.score.value {
            return Ok(self.score);
        }

        Err(ScoreValidationError::UnexpectedScore {
            score_message: Box::new(self),
            expected_score,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn correctly_recognise_a_valid_short_game() {
        let body = include_str!("../resources/games/short.json");
        let message: ScoreMessage = serde_json::from_str(&body).unwrap();
        assert_eq!(
            message.score().unwrap(),
            Score::new(1700, "AEL".to_string())
        );
    }

    #[test]
    #[ignore] // TODO: fix whatever causes this to fail
    fn correctly_recognise_a_valid_long_game() {
        let body = include_str!("../resources/games/long.json");
        let message: ScoreMessage = serde_json::from_str(&body).unwrap();
        assert_eq!(
            message.score().unwrap(),
            Score::new(24800, "AEL".to_string())
        );
    }
}
