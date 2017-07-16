use score::Score;
use err::Result;
use score::ScoreMessage;
use serde_json;

#[cfg(not(target_os = "emscripten"))]
use hyper;


const HI_SCORES_ENDPOINT: &'static str = "http://tetris.ael.red/scores";


impl Client {
    pub fn get_hiscores(&mut self) -> Result<Vec<Score>> {
        let body = try!(self.get_raw_hiscores());
        let hiscores = try!(serde_json::from_str(&body));
        Ok(hiscores)
    }

    pub fn post_hiscore(&mut self, score: &ScoreMessage) {
        let body = serde_json::to_string(score).unwrap();
        let response = self.post_raw_hiscores(body);

        if let Err(e) = response {
            println!("Failed to post hiscores: {}", e);
        }
    }
}

#[cfg(not(target_os = "emscripten"))]
pub struct Client {
    hyper_client: hyper::client::Client,
}

#[cfg(not(target_os = "emscripten"))]
impl Default for Client {
    fn default() -> Self {
        Client { hyper_client: hyper::client::Client::new() }
    }
}

#[cfg(not(target_os = "emscripten"))]
impl Client {
    fn get_raw_hiscores(&mut self) -> Result<String> {
        use std::io::Read;

        let mut body = String::new();
        let mut res = try!(self.hyper_client.get(HI_SCORES_ENDPOINT).send());
        try!(res.read_to_string(&mut body));
        Ok(body)
    }

    fn post_raw_hiscores(&mut self, score: String) -> Result<()> {
        try!(
            self.hyper_client
                .post(HI_SCORES_ENDPOINT)
                .body(score.as_bytes())
                .send()
        );
        Ok(())
    }
}

#[cfg(target_os = "emscripten")]
pub struct Client;

#[cfg(target_os = "emscripten")]
impl Default for Client {
    fn default() -> Self {
        Client
    }
}

#[cfg(target_os = "emscripten")]
impl Client {
    fn get_raw_hiscores(&self) -> Result<String> {
        use emscripten::em;

        let script = format!(
            r#"(function() {{
            var req = new XMLHttpRequest();
            req.open("GET", "{}", false);
            req.send(null);
            return req.responseText;
        }}())"#,
            HI_SCORES_ENDPOINT
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
            HI_SCORES_ENDPOINT,
            score
        );

        em::run_script(&script);
        Ok(())
    }
}
