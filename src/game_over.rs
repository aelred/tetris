use draw::TextDrawer;
use draw::Drawer;
use state::State;
use state::StateChange;
use lib::score::Score;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


const USE_HI_SCORES: bool = false;


pub struct GameOver {
    hi_scores: Vec<Score>,
    score: Score,
}

impl GameOver {
    pub fn new(score: u32) -> Self {
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
        let offset = 100;

        let mut text = text.size(3)
            .under()
            .offset(0, 10)
            .draw("High Scores");

        text = text.size(2).under().offset(0, 10);

        for &Score { ref value, ref name } in &self.hi_scores {
            text = text.offset(-offset, 0)
                .draw(&name)
                .offset(offset * 2, 0)
                .draw(&value.to_string())
                .under()
                .offset(-offset, 10);
        }

        text.under().offset(-offset, 10)
    }
}

fn get_hiscores() -> Vec<Score> {
    vec![Score::new(1000, "FEL".to_string()),
         Score::new(900, "ANG".to_string()),
         Score::new(800, "LLY".to_string()),
         Score::new(700, "MKO".to_string()),
         Score::new(600, "ALX".to_string()),
         Score::new(500, "JSN".to_string()),
         Score::new(400, "SHD".to_string()),
         Score::new(300, "CHR".to_string()),
         Score::new(200, "SRH".to_string()),
         Score::new(100, "EMY".to_string())]
}
