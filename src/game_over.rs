use draw::TextDrawer;
use draw::Drawer;
use state::State;
use state::StateChange;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub struct GameOver {
    hi_scores: Vec<HiScore>,
    score: u32,
}

impl GameOver {
    pub fn new(score: u32) -> Self {
        GameOver {
            hi_scores: get_hiscores(),
            score: score,
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

        let mut text = drawer.text()
            .size(3)
            .top(50)
            .draw("Game Over")
            .under(10)
            .size(1)
            .draw("final score")
            .under(0)
            .size(3)
            .draw(&self.score.to_string());

        text = draw_hiscores(&self.hi_scores, text);

        text.size(1).draw("[ Press Enter ]");

        StateChange::None
    }
}

struct HiScore {
    score: u32,
    name: String,
}

fn get_hiscores() -> Vec<HiScore> {
    vec![HiScore {
             score: 0,
             name: "FEL".to_string(),
         },
         HiScore {
             score: 100,
             name: "ANG".to_string(),
         },
         HiScore {
             score: 200,
             name: "LLY".to_string(),
         },
         HiScore {
             score: 300,
             name: "MKO".to_string(),
         },
         HiScore {
             score: 400,
             name: "ALX".to_string(),
         },
         HiScore {
             score: 500,
             name: "JSN".to_string(),
         },
         HiScore {
             score: 600,
             name: "SHD".to_string(),
         },
         HiScore {
             score: 700,
             name: "CHR".to_string(),
         },
         HiScore {
             score: 800,
             name: "SRH".to_string(),
         },
         HiScore {
             score: 900,
             name: "EMY".to_string(),
         }]
}

fn draw_hiscores<'a, 'b>(hiscores: &Vec<HiScore>, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b> {

    let offset = 100;

    let mut text = text.size(3).under(10).draw("High Scores");

    text = text.size(2).under(10);

    for &HiScore { ref score, ref name } in hiscores {
        text = text.offset(-offset, 0)
            .draw(&name)
            .offset(offset * 2, 0)
            .draw(&score.to_string())
            .under(10)
            .offset(-offset, 0);
    }

    text.under(10).offset(-offset, 0)
}
