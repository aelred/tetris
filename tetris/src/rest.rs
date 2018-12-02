use std::error::Error;

use hyper;
use lazy_static::lazy_static;
use serde_json;
use url;
use url::Url;

use crate::score::{Score, SCORE_ENDPOINT};
use crate::score::ScoreMessage;

lazy_static! {
    static ref CLIENT: Client = Client::new(Url::parse("http://tetris.ael.red").unwrap());
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn get_hiscores() -> Result<Vec<Score>> {
    let body = CLIENT.get_raw_hiscores()?;
    let hiscores = serde_json::from_str(&body)?;
    Ok(hiscores)
}

pub fn post_hiscore(score: &ScoreMessage) {
    let body = serde_json::to_string(score).unwrap();
    let response = CLIENT.post_raw_hiscores(&body);

    if let Err(e) = response {
        println!("Failed to post hiscores: {}", e);
    }
}

impl Client {
    fn scores_endpoint(&self) -> Url {
        self.url.join(SCORE_ENDPOINT).unwrap()
    }
}

struct Client {
    url: Url,
    hyper_client: hyper::client::Client,
}

impl Client {
    fn new(url: Url) -> Self {
        Client {
            url,
            hyper_client: hyper::client::Client::new(),
        }
    }

    fn get_raw_hiscores(&self) -> Result<String> {
        use std::io::Read;

        let mut body = String::new();
        let mut res = self.hyper_client.get(self.scores_endpoint()).send()?;
        res.read_to_string(&mut body)?;
        Ok(body)
    }

    fn post_raw_hiscores(&self, score: &str) -> Result<()> {
        self.hyper_client
            .post(self.scores_endpoint())
            .body(score.as_bytes())
            .send()?;
        Ok(())
    }
}
