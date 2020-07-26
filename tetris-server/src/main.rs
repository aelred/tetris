#![feature(proc_macro_hygiene, decl_macro)]

use std::error::Error;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;
use tetris::Score;
use tetris::ScoreMessage;

#[macro_use]
extern crate rocket;

struct ScoresHandler {
    hiscores: RwLock<Vec<Score>>,
    hiscores_path: PathBuf,
}

impl ScoresHandler {
    fn new(hiscores_path: PathBuf) -> ScoresHandler {
        let hiscores = match File::open(&hiscores_path) {
            Ok(mut file) => read_hiscores(&mut file),
            Err(_) => init_hiscores(),
        };

        ScoresHandler {
            hiscores: RwLock::new(hiscores),
            hiscores_path,
        }
    }

    fn add_hiscore(&self, message: ScoreMessage) -> Result<Vec<Score>, Box<dyn Error>> {
        let score = message.score()?;

        {
            let hiscores = &mut (*self.hiscores.write().unwrap());
            hiscores.push(score);
            hiscores.sort();
            hiscores.pop();

            let mut file = File::create(&self.hiscores_path)?;
            file.write_all(serde_json::to_string(hiscores).unwrap().as_bytes())?;
        }

        Ok(self.get_hiscores())
    }

    fn get_hiscores(&self) -> Vec<Score> {
        self.hiscores.read().unwrap().clone()
    }
}

#[get("/score")]
fn get_scores(scores: State<ScoresHandler>) -> Json<Vec<Score>> {
    Json(scores.get_hiscores())
}

#[post("/score", data = "<message>")]
fn post_score(
    message: Json<ScoreMessage>,
    scores: State<ScoresHandler>,
) -> Result<Json<Vec<Score>>, String> {
    scores
        .add_hiscore(message.0)
        .map(Json)
        .map_err(|e| e.to_string())
}

fn main() {
    let path = hiscores_path();
    let scores = ScoresHandler::new(path);
    rocket::ignite()
        .manage(scores)
        .mount("/", routes![get_scores, post_score])
        .launch();
}

fn read_hiscores(file: &mut File) -> Vec<Score> {
    let mut hiscores = String::new();
    file.read_to_string(&mut hiscores)
        .expect("Hiscores file is invalid");
    serde_json::from_str(&hiscores).expect("Hiscores file is invalid")
}

fn init_hiscores() -> Vec<Score> {
    let mut hiscores = Vec::new();
    for _ in 0..10 {
        hiscores.push(Score::new(0, "AEL".to_string()));
    }
    hiscores
}

const TETRIS_CONF: &str = ".tetris";

fn hiscores_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(TETRIS_CONF);

    let _ = DirBuilder::new().create(&path);

    path.push("hiscores.json");
    path
}
