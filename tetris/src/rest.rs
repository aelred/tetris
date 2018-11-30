extern crate url;

use score::ScoreMessage;
use score::{Score, SCORE_ENDPOINT};
use serde_json;
use url::Url;

#[cfg(not(target_os = "emscripten"))]
use hyper;

lazy_static! {
    static ref CLIENT: Client = Client::new(Url::parse("http://tetris.ael.red").unwrap());
}

type Result<T> = std::result::Result<T, Box<Error>>;

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
use libc;
use std::error::Error;

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn emscripten_run_script(script: *const libc::c_char);
    pub fn emscripten_run_script_string(script: *const libc::c_char) -> *mut libc::c_char;
}

#[cfg(target_os = "emscripten")]
impl Client {
    fn new(url: Url) -> Self {
        Client { url }
    }

    fn run_script(script: &str) {
        use std::ffi::CString;

        let script = CString::new(script).unwrap();
        unsafe {
            emscripten_run_script(script.as_ptr());
        }
    }

    fn run_script_string(script: &str) -> String {
        use std::ffi::{CStr, CString};

        let script = CString::new(script).unwrap();
        unsafe {
            let ptr = emscripten_run_script_string(script.as_ptr());
            let c_str = CStr::from_ptr(ptr);
            String::from(c_str.to_str().unwrap())
        }
    }

    fn get_raw_hiscores(&self) -> Result<String> {
        let script = format!(
            r#"(function() {{
            var req = new XMLHttpRequest();
            req.open("GET", "{}", false);
            req.send(null);
            return req.responseText;
        }}())"#,
            self.scores_endpoint()
        );

        Ok(Client::run_script_string(&script))
    }

    fn post_raw_hiscores(&self, score: &str) -> Result<()> {
        let script = format!(
            r#"(function() {{
            var req = new XMLHttpRequest();
            req.open("POST", "{}", false);
            req.send(JSON.stringify({}));
        }}())"#,
            self.scores_endpoint(),
            score
        );

        Client::run_script(&script);
        Ok(())
    }
}
