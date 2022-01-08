#![feature(proc_macro_hygiene, decl_macro)]

use std::error::Error;
use std::fs::{DirBuilder, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use fslock::LockFile;

use rocket::State;
use rocket_contrib::json::Json;
use tetris::Score;
use tetris::ScoreMessage;

#[macro_use]
extern crate rocket;

struct ScoresHandler {
    scores_path: PathBuf,
    lock_path: PathBuf
}

impl ScoresHandler {
    fn new(scores_path: PathBuf, lock_path: PathBuf) -> ScoresHandler {
        ScoresHandler { scores_path, lock_path }
    }

    fn add_score(&self, message: ScoreMessage) -> Result<Vec<Score>, Box<dyn Error>> {
        let score = message.score()?;

        let mut lock = LockFile::open(&self.lock_path)?;
        lock.lock()?;

        let mut scores = self.get_scores();
        scores.push(score);
        scores.sort();
        scores.pop();

        let mut file = File::create(&self.scores_path)?;
        file.write_all(serde_json::to_string(&scores).unwrap().as_bytes())?;

        lock.unlock()?;

        Ok(scores)
    }

    fn get_scores(&self) -> Vec<Score> {
        match File::open(&self.scores_path) {
            Ok(mut file) => {
                let mut scores = String::new();
                file.read_to_string(&mut scores).expect("scores file is invalid");
                serde_json::from_str(&scores).expect("scores file is invalid")
            },
            Err(_) => {
                let mut scores = Vec::new();
                for _ in 0..10 {
                    scores.push(Score::new(0, "AEL".to_string()));
                }
                scores
            }
        }
    }
}

#[get("/scores")]
fn get_scores(scores: State<ScoresHandler>) -> Json<Vec<Score>> {
    Json(scores.get_scores())
}

#[post("/scores", data = "<message>")]
fn post_score(
    message: Json<ScoreMessage>,
    scores: State<ScoresHandler>,
) -> Result<Json<Vec<Score>>, String> {
    scores
        .add_score(message.0)
        .map(Json)
        .map_err(|e| e.to_string())
}

fn main() {
    let mut conf_dir = dirs::home_dir().unwrap();
    conf_dir.push(TETRIS_CONF);
    let _ = DirBuilder::new().create(&conf_dir);
    rocket(conf_dir).launch();
}

fn rocket(conf_dir: impl Into<PathBuf>) -> rocket::Rocket {
    let conf_dir = conf_dir.into();
    let mut scores_path = conf_dir.clone();
    scores_path.push("hiscores.json");
    let mut lock_path = conf_dir;
    lock_path.push("hiscores.json.lock");
    let scores = ScoresHandler::new(scores_path, lock_path);
    rocket::ignite().manage(scores).mount("/", routes![get_scores, post_score])
}

const TETRIS_CONF: &str = ".tetris";

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;
    use serde_json::{json, Value};
    use tempdir::TempDir;

    const GAME: &str = include_str!("../../resources/games/short.json");

    #[test]
    fn scores_begin_empty() {
        let client = client_from_dir(new_config_dir().into_path());
        assert_eq!(get_scores(&client), empty_scores());
    }

    #[test]
    fn valid_scores_are_added() {
        let client = client_from_dir(new_config_dir().into_path());
        assert_eq!(post_score(&client, GAME), short_score());
        assert_eq!(get_scores(&client), short_score());
    }

    #[test]
    fn scores_are_persisted() {
        let config_dir = new_config_dir();
        {
            let client1 = client_from_dir(config_dir.path());
            post_score(&client1, GAME);
        }
        {
            let client2 = client_from_dir(config_dir.path());
            assert_eq!(get_scores(&client2), short_score());
        }
    }

    #[test]
    fn scores_are_consistent_between_replicas() {
        let config_dir = new_config_dir();
        let client1 = client_from_dir(config_dir.path());
        let client2 = client_from_dir(config_dir.path());
        post_score(&client1, GAME);
        assert_eq!(get_scores(&client2), short_score());
    }

    fn client_from_dir(config_dir: impl Into<PathBuf>) -> Client {
        Client::new(rocket(config_dir)).expect("valid rocket instance")
    }

    fn new_config_dir() -> TempDir {
        TempDir::new("tetris-server").expect("temp config dir")
    }

    fn get_scores(client: &Client) -> Value {
        let mut response = client.get("/scores").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.body_string().expect("body");
        serde_json::from_str::<Value>(&body).expect("json")
    }

    fn post_score(client: &Client, body: &str) -> Value {
        let mut response = client.post("/scores").body(body).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.body_string().expect("Expected body in response");
        serde_json::from_str::<Value>(&body).expect("Expected valid JSON response")
    }

    fn empty_scores()-> Value {
        json!([
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0}
        ])
    }

    fn short_score() -> Value {
        json!([
            {"name": "SHT", "value": 1700},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0},
            {"name": "AEL", "value": 0}
        ])
    }
}