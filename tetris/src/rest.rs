use std::error::Error;

use lazy_static::lazy_static;
#[cfg(target_os = "emscripten")]
use libc;
#[cfg(not(target_os = "emscripten"))]
use url::Url;

use crate::score::ScoreMessage;
use crate::score::{Score, SCORE_ENDPOINT};

lazy_static! {
    static ref CLIENT: Client = Client::new();
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

#[cfg(not(target_os = "emscripten"))]
struct Client {
    url: Url,
    reqwest_client: reqwest::blocking::Client,
}

#[cfg(not(target_os = "emscripten"))]
impl Client {
    fn new() -> Self {
        Client {
            url: Url::parse("https://tetris.ael.red").unwrap(),
            reqwest_client: reqwest::blocking::Client::new(),
        }
    }

    fn scores_endpoint(&self) -> Url {
        self.url.join(SCORE_ENDPOINT).unwrap()
    }

    fn get_raw_hiscores(&self) -> Result<String> {
        use std::io::Read;

        let mut body = String::new();
        let mut res = self.reqwest_client.get(self.scores_endpoint()).send()?;
        res.read_to_string(&mut body)?;
        Ok(body)
    }

    fn post_raw_hiscores(&self, score: &str) -> Result<()> {
        self.reqwest_client
            .post(self.scores_endpoint())
            .body(score.to_string())
            .send()?;
        Ok(())
    }
}

#[cfg(target_os = "emscripten")]
struct Client;

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn emscripten_run_script(script: *const libc::c_char);
    pub fn emscripten_run_script_string(script: *const libc::c_char) -> *mut libc::c_char;
}

#[cfg(target_os = "emscripten")]
impl Client {
    fn new() -> Self {
        Client
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
            SCORE_ENDPOINT
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
            SCORE_ENDPOINT, score
        );

        Client::run_script(&script);
        Ok(())
    }
}
