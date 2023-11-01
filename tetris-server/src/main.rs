use fslock::LockFile;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use std::env;
use std::error::Error;
use std::fs::{DirBuilder, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use rocket::State;
use tetris::Score;
use tetris::ScoreMessage;

#[macro_use]
extern crate rocket;

struct ScoresHandler {
    scores_path: PathBuf,
    lock_path: PathBuf,
}

impl ScoresHandler {
    fn new(scores_path: PathBuf, lock_path: PathBuf) -> ScoresHandler {
        ScoresHandler {
            scores_path,
            lock_path,
        }
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
                file.read_to_string(&mut scores)
                    .expect("scores file is invalid");
                serde_json::from_str(&scores).expect("scores file is invalid")
            }
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
fn get_scores(scores: &State<ScoresHandler>) -> Json<Vec<Score>> {
    Json(scores.get_scores())
}

#[post("/scores", data = "<message>")]
fn post_score(
    message: Json<ScoreMessage>,
    scores: &State<ScoresHandler>,
) -> Result<Json<Vec<Score>>, String> {
    scores
        .add_score(message.0)
        .map(Json)
        .map_err(|e| e.to_string())
}

#[launch]
fn start() -> _ {
    let mut conf_dir = dirs::home_dir().unwrap();
    conf_dir.push(TETRIS_CONF);
    let _ = DirBuilder::new().create(&conf_dir);
    rocket(conf_dir)
}

fn rocket(conf_dir: impl Into<PathBuf>) -> rocket::Rocket<rocket::Build> {
    let conf_dir = conf_dir.into();
    let mut scores_path = conf_dir.clone();
    scores_path.push("hiscores.json");
    let mut lock_path = conf_dir;
    lock_path.push("hiscores.json.lock");
    let scores = ScoresHandler::new(scores_path, lock_path);

    let static_path = env::var("STATIC_FILES")
        .or_else(|_| env::var("CARGO_MANIFEST_DIR").map(|s| s + "/../static"))
        .expect("Expected STATIC_FILES or CARGO_MANIFEST_DIR to be set");

    rocket::build()
        .manage(scores)
        .mount("/", FileServer::from(static_path))
        .mount("/", routes![get_scores, post_score])
}

const TETRIS_CONF: &str = ".tetris";

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use serde_json::{json, Value};
    use std::path::PathBuf;
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
        Client::untracked(rocket(config_dir)).expect("valid rocket instance")
    }

    fn new_config_dir() -> TempDir {
        TempDir::new("tetris-server").expect("temp config dir")
    }

    fn get_scores(client: &Client) -> Value {
        let response = client.get("/scores").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().expect("body");
        serde_json::from_str::<Value>(&body).expect("json")
    }

    fn post_score(client: &Client, body: &str) -> Value {
        let response = client.post("/scores").body(body).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().expect("Expected body in response");
        serde_json::from_str::<Value>(&body).expect("Expected valid JSON response")
    }

    fn empty_scores() -> Value {
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
