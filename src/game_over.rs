use draw::TextDrawer;
use draw::Drawer;
use state::State;
use state::StateChange;
use score::Score;
use std::io::Read;
use std::error::Error;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use hyper::client::Client;
use rustc_serialize::json;


const USE_HI_SCORES: bool = false;

const HI_SCORES_ENDPOINT: &'static str = "http://tetris.ael.red/scores";


pub struct GameOver {
    hi_scores: Result<Vec<Score>, Box<Error>>,
    score: Score,
}

impl GameOver {
    pub fn new(score: u32) -> Self {
        let hiscores = get_hiscores();

        if let Err(e) = hiscores {
            println!("{}", e);
        }

        GameOver {
            hi_scores: get_hiscores(),
            score: Score::new(score, "".to_string()),
        }
    }

    pub fn update(&self, drawer: &mut Drawer, events: &[Event]) -> StateChange {
        for event in events {
            if let Event::KeyDown { keycode: Some(keycode), .. } = *event {
                if let Keycode::Return = keycode {
                    return StateChange::Replace(State::play());
                }
            }
        }

        let mut text = drawer.text();

        text = if USE_HI_SCORES {
            text.top().offset(0, 50)
        } else {
            text.centered()
        };

        let mut text = text.size(3)
            .draw("Game Over")
            .under()
            .offset(0, 10)
            .size(1)
            .draw("final score")
            .under()
            .size(3)
            .draw(&self.score.value.to_string());

        text = if USE_HI_SCORES {
            self.draw_hiscores(text)
        } else {
            text.under().offset(0, 10)
        };

        text.size(1).draw("[ Press Enter ]");

        StateChange::None
    }

    fn draw_hiscores<'a, 'b>(&self, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b> {

        match self.hi_scores {
            Ok(ref hi_scores) => {
                let offset = 100;

                let mut text = text.size(3)
                    .under()
                    .offset(0, 10)
                    .draw("High Scores");

                text = text.size(2).under().offset(0, 10);

                for &Score { ref value, ref name } in hi_scores {
                    text = text.offset(-offset, 0)
                        .draw(&name)
                        .offset(offset * 2, 0)
                        .draw(&value.to_string())
                        .under()
                        .offset(-offset, 10);
                }

                text.under().offset(-offset, 10)
            }
            Err(_) => {
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
    let client = Client::new();
    let mut body = String::new();
    let mut res = try!(client.get(HI_SCORES_ENDPOINT).send());
    try!(res.read_to_string(&mut body));

    let hiscores = try!(json::decode(&body));

    Ok(hiscores)
}
