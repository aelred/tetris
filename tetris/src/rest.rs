extern crate url;

use score::{Score, SCORE_ENDPOINT};
use err::Result;
use score::ScoreMessage;
use serde_json;
use url::Url;

#[cfg(not(target_os = "emscripten"))]
use hyper;


lazy_static! {
    static ref CLIENT: Client = Client::new(Url::parse("http://tetris.ael.red").unwrap());
}


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

#[cfg(not(target_os = "emscripten"))]
struct Client {
    url: Url,
    hyper_client: hyper::client::Client,
}

#[cfg(not(target_os = "emscripten"))]
impl Client {
    fn new(url: Url) -> Self {
        Client {
            url,
            hyper_client: hyper::client::Client::new(),
        }
    }

    fn get_raw_hiscores(&self) -> Result<String> {
        use std::io::Read;

        println!("{}", self.scores_endpoint());
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

#[cfg(target_os = "emscripten")]
struct Client {
    url: Url,
}

#[cfg(target_os = "emscripten")]
impl Client {
    fn new(url: Url) -> Self {
        Client { url }
    }

    fn get_raw_hiscores(&self) -> Result<String> {
        use emscripten::em;

        let script = format!(
            r#"(function() {{
            var req = new XMLHttpRequest();
            req.open("GET", "{}", false);
            req.send(null);
            return req.responseText;
        }}())"#,
            self.scores_endpoint()
        );

        Ok(em::run_script_string(&script))
    }

    fn post_raw_hiscores(&self, score: String) -> Result<()> {
        use emscripten::em;

        let script = format!(
            r#"(function() {{
            var req = new XMLHttpRequest();
            req.open("POST", "{}", false);
            req.send(JSON.stringify({}));
        }}())"#,
            self.scores_endpoint(),
            score
        );

        em::run_script(&script);
        Ok(())
    }
}