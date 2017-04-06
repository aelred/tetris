extern crate lib;
extern crate hyper;
extern crate rustc_serialize;

use lib::score::Score;

use rustc_serialize::json;

use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::sync::RwLock;
use std::io::Read;

use hyper::{Get, Post};
use hyper::server::{Server, Handler, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use hyper::header::ContentType;
use hyper::status::StatusCode;

const TETRIS_DATA: &'static str = "/var/lib/tetris";
const SCORES_PATH: &'static str = "/scores.json";

macro_rules! print_err(
    ($e:expr) => {{
        if let Err(e) = $e {
            println!("{}", e);
        }
    }}
);

struct ScoresHandler {
    hiscores: RwLock<Vec<Score>>,
}

impl ScoresHandler {
    fn add_hiscore(&self, req: &mut Request, mut res: Response) -> Result<(), Box<Error>> {
        let mut body = String::new();
        req.read_to_string(&mut body)?;

        let score: Score = match json::decode(&body) {
            Ok(s) => s,
            Err(_) => {
                *res.status_mut() = StatusCode::BadRequest;
                return Ok(());
            }
        };

        if !score.name.chars().all(char::is_alphanumeric) || score.name.chars().count() > 3 {
            *res.status_mut() = StatusCode::BadRequest;
            return Ok(());
        }

        {
            let ref mut hiscores = *self.hiscores.write().unwrap();
            hiscores.push(score);
            hiscores.sort_by_key(|s| std::u32::MAX - s.value);
            hiscores.pop();

            let mut file = try!(File::create(format!("{}{}", TETRIS_DATA, SCORES_PATH)));
            file.write_all(json::encode(hiscores).unwrap().as_bytes())?;
        }

        *res.status_mut() = StatusCode::Created;

        self.send_hiscores(res)?;
        Ok(())
    }

    fn send_hiscores(&self, mut res: Response) -> std::io::Result<()> {
        let ref hiscores = *self.hiscores.read().unwrap();

        res.headers_mut().set(ContentType::json());
        let body = json::encode(hiscores).unwrap();
        res.send(body.as_bytes())
    }
}

impl Handler for ScoresHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        if let AbsolutePath(path) = req.uri.clone() {
            match (&req.method, &path[..]) {
                (&Get, "/scores") => {
                    print_err!(self.send_hiscores(res));
                }
                (&Post, "/scores") => {
                    print_err!(self.add_hiscore(&mut req, res));
                }
                _ => {
                    *res.status_mut() = hyper::NotFound;
                }
            }
        };

        return;
    }
}

fn main() {
    let _ = std::fs::create_dir(TETRIS_DATA);

    let server = Server::http("localhost:4444").unwrap();

    let handler = ScoresHandler { hiscores: RwLock::new(init_hiscores()) };

    server.handle(handler).unwrap();
}

fn init_hiscores() -> Vec<Score> {
    let mut hiscores = Vec::new();
    for _ in 0..10 {
        hiscores.push(Score::new(0, "AEL".to_string()));
    }
    hiscores
}
