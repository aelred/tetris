use draw::TextDrawer;
use draw::Drawer;
use state::State;
use state::StateChange;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


const USE_HI_SCORES: bool = false;


pub struct GameOver {
    hi_scores: Vec<HiScore>,
    score: HiScore,
}

impl GameOver {
    pub fn new(score: u32) -> Self {
        GameOver {
            hi_scores: get_hiscores(),
            score: HiScore {
                score: score,
                name: "".to_string(),
            },
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
            .draw(&self.score.score.to_string());

        text = if USE_HI_SCORES {
            self.draw_hiscores(text)
        } else {
            text.under().offset(0, 10)
        };

        text.size(1).draw("[ Press Enter ]");

        StateChange::None
    }

    fn draw_hiscores<'a, 'b>(&self, text: TextDrawer<'a, 'b>) -> TextDrawer<'a, 'b> {
        let offset = 100;

        let mut text = text.size(3)
            .under()
            .offset(0, 10)
            .draw("High Scores");

        text = text.size(2).under().offset(0, 10);

        for &HiScore { ref score, ref name } in &self.hi_scores {
            text = text.offset(-offset, 0)
                .draw(&name)
                .offset(offset * 2, 0)
                .draw(&score.to_string())
                .under()
                .offset(-offset, 10);
        }

        text.under().offset(-offset, 10)
    }
}

struct HiScore {
    score: u32,
    name: String,
}

fn get_hiscores() -> Vec<HiScore> {
    vec![HiScore {
             score: 1000,
             name: "FEL".to_string(),
         },
         HiScore {
             score: 900,
             name: "ANG".to_string(),
         },
         HiScore {
             score: 800,
             name: "LLY".to_string(),
         },
         HiScore {
             score: 700,
             name: "MKO".to_string(),
         },
         HiScore {
             score: 600,
             name: "ALX".to_string(),
         },
         HiScore {
             score: 500,
             name: "JSN".to_string(),
         },
         HiScore {
             score: 400,
             name: "SHD".to_string(),
         },
         HiScore {
             score: 300,
             name: "CHR".to_string(),
         },
         HiScore {
             score: 200,
             name: "SRH".to_string(),
         },
         HiScore {
             score: 100,
             name: "EMY".to_string(),
         }]
}
