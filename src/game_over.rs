use draw::TextDrawer;
use draw::Drawer;
use state::State;
use state::StateChange;
use score::Score;
use score::OFFSET;
use std::error::Error;

use regex::Regex;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rustc_serialize::json;


const HI_SCORES_ENDPOINT: &'static str = "http://tetris.ael.red/scores";


pub struct GameOver {
    hiscores: Option<HighScores>,
    score: Score,
    posting_hiscore: bool,
}

struct HighScores {
    higher_scores: Vec<Score>,
    lower_scores: Vec<Score>,
}

impl HighScores {
    fn new(hiscores: Vec<Score>, user_score: &Score) -> Self {
        let index = match hiscores.binary_search(user_score) {
            Ok(i) => i,
            Err(i) => i,
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
        self.lower_scores.len() != 0
    }
}

impl GameOver {
    pub fn new(score: u32) -> Self {
        let hiscores = get_hiscores();

        if let &Err(ref e) = &hiscores {
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
            posting_hiscore: posting_hiscore,
        }
    }

    pub fn update(&mut self, drawer: &mut Drawer, events: &[Event]) -> StateChange {

        lazy_static! {
            static ref ALPHANUMERIC: Regex = Regex::new("^[a-zA-Z0-9]$").unwrap();
        }

        for event in events {
            if let Event::KeyDown { keycode: Some(keycode), .. } = *event {
                match keycode {
                    Keycode::Return => {
                        if !self.posting_hiscore || !self.score.name.is_empty() {
                            if self.posting_hiscore {
                                post_hiscore(&self.score);
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
        }

        let mut text = drawer.text()
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
            Some(HighScores { ref higher_scores, ref lower_scores }) => {
                let mut text = text.size(3)
                    .under()
                    .offset(0, 10)
                    .draw("High Scores");

                text = text.size(2).under().offset(0, 10);

                for score in higher_scores {
                    text = score.draw(text);
                }

                if self.posting_hiscore {
                    text = self.score.draw(text.color(Color::RGB(255, 255, 100))).reset_color();
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

fn get_hiscores() -> Result<Vec<Score>, Box<Error>> {
    let body = try!(get_raw_hiscores());
    let hiscores = try!(json::decode(&body));
    Ok(hiscores)
}

fn post_hiscore(score: &Score) {
    let body = json::encode(score).unwrap();
    let response = post_raw_hiscores(body);

    if let Err(e) = response {
        println!("Failed to post hiscores: {}", e);
    }
}

#[cfg(not(target_os="emscripten"))]
fn get_raw_hiscores() -> Result<String, Box<Error>> {
    use hyper::client::Client;
    use std::io::Read;

    let client = Client::new();
    let mut body = String::new();
    let mut res = try!(client.get(HI_SCORES_ENDPOINT).send());
    try!(res.read_to_string(&mut body));
    Ok(body)
}

#[cfg(target_os="emscripten")]
fn get_raw_hiscores() -> Result<String, Box<Error>> {
    use emscripten::em;

    let script = format!(r#"(function() {{
        var req = new XMLHttpRequest();
        req.open("GET", "{}", false);
        req.send(null);
        return req.responseText;
    }}())"#,
                         HI_SCORES_ENDPOINT);

    Ok(em::run_script_string(&script))
}

#[cfg(not(target_os="emscripten"))]
fn post_raw_hiscores(score: String) -> Result<(), Box<Error>> {
    use hyper::client::Client;

    let client = Client::new();
    let body = json::encode(&score).unwrap();
    try!(client.post(HI_SCORES_ENDPOINT).body(body.as_bytes()).send());
    Ok(())
}

#[cfg(target_os="emscripten")]
fn post_raw_hiscores(score: String) -> Result<(), Box<Error>> {
    use emscripten::em;

    let script = format!(r#"(function() {{
        var req = new XMLHttpRequest();
        req.open("POST", "{}", false);
        req.send(JSON.stringify({}));
    }}())"#,
                         HI_SCORES_ENDPOINT,
                         score);

    em::run_script(&script);
    Ok(())
}
