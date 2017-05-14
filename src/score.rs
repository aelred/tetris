use draw::TextDrawer;
use game::History;
use err::Result;
use std::cmp::Ordering;

pub const OFFSET: i32 = 100;

pub const VERIFY_SCORES: bool = false;

#[derive(RustcDecodable, RustcEncodable, Eq, PartialEq, Clone)]
pub struct Score {
    pub value: u32,
    pub name: String,
}

impl Score {
    pub fn new(value: u32, name: String) -> Self {
        Score {
            value: value,
            name: name,
        }
    }

    pub fn draw<'a, 'b>(&self, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b> {
        let name = if self.name.is_empty() {
            " "
        } else {
            &self.name
        };

        text.offset(-OFFSET, 0)
            .draw(name)
            .offset(OFFSET * 2, 0)
            .draw(&self.value.to_string())
            .under()
            .offset(-OFFSET, 10)
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

#[derive(RustcDecodable, RustcEncodable)]
pub struct ScoreMessage {
    score: Score,
    history: History,
}

impl ScoreMessage {
    pub fn new(score: Score, history: History) -> Self {
        ScoreMessage {
            score: score,
            history: history,
        }
    }

    pub fn score(self) -> Result<Score> {
        if self.score.name.is_empty() {
            return Err(From::from("Name should not be empty"));
        }

        if self.score.name.len() > 3 {
            let message = format!("Name must be at most 3 characters, but was {}",
                                  self.score.name.len());
            return Err(From::from(message));
        }

        if !self.score
                .name
                .chars()
                .all(char::is_alphanumeric) {
            let message = format!("Name must contain only alphanumeric characters, but was {}",
                                  self.score.name);
            return Err(From::from(message));
        }

        if VERIFY_SCORES {
            let expected_score = self.history.replay();
            if expected_score != self.score.value {
                let message = format!("Score does not match game history: History suggests {} but was {}",
                                      expected_score,
                                      self.score.value);
                return Err(From::from(message));

            }
        }

        Ok(self.score)
    }
}
