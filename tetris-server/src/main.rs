extern crate hyper;
extern crate serde_json;
extern crate tetris;

#[macro_use]
extern crate clap;

use tetris::score::SCORE_ENDPOINT;
use tetris::score::Score;
use tetris::score::ScoreMessage;
use tetris::err::Result;

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::sync::RwLock;
use std::io::Read;
use std::fs::DirBuilder;

use clap::{App, Arg};

use hyper::{Get, Post};
use hyper::server::{Server, Handler, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use hyper::header::{ContentType, AccessControlAllowOrigin};
use hyper::status::StatusCode;

macro_rules! print_err (
    ($e:expr) => {{
        if let Err(e) = $e {
            println!("{}", e);
        }
    }}
);

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

    fn decode_score_message(body: &str) -> Result<Score> {
        let message: ScoreMessage = serde_json::from_str(body)?;
        let score = message.score()?;
        Ok(score)
    }

    fn add_hiscore(&self, req: &mut Request, mut res: Response) -> Result<()> {
        let mut body = String::new();
        req.read_to_string(&mut body)?;

        let score: Score = match ScoresHandler::decode_score_message(&body) {
            Ok(s) => s,
            Err(e) => {
                println!("Bad request: {} - {}", &body, e);
                *res.status_mut() = StatusCode::BadRequest;
                res.send(e.to_string().as_bytes())?;
                return Ok(());
            }
        };

        {
            let hiscores = &mut (*self.hiscores.write().unwrap());
            hiscores.push(score);
            hiscores.sort();
            hiscores.pop();

            let mut file = File::create(&self.hiscores_path)?;
            file.write_all(
                serde_json::to_string(hiscores).unwrap().as_bytes(),
            )?;
        }

        *res.status_mut() = StatusCode::Created;

        self.send_hiscores(res)?;
        Ok(())
    }

    fn send_hiscores(&self, mut res: Response) -> std::io::Result<()> {
        let hiscores = &(*self.hiscores.read().unwrap());

        res.headers_mut().set(ContentType::json());
        let body = serde_json::to_string(hiscores).unwrap();
        res.send(body.as_bytes())
    }
}

impl Handler for ScoresHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        res.headers_mut().set(AccessControlAllowOrigin::Any);

        if let AbsolutePath(path) = req.uri.clone() {
            match (&req.method, &path[..]) {
                (&Get, SCORE_ENDPOINT) => {
                    print_err!(self.send_hiscores(res));
                }
                (&Post, SCORE_ENDPOINT) => {
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
    let port_arg = "PORT";

    let matches = App::new("tetris-server")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name(port_arg)
                .short("p")
                .long("port")
                .default_value("4444")
                .help("Set the port to use"),
        )
        .get_matches();

    let port = value_t!(matches, "PORT", u16).unwrap_or_else(|e| e.exit());

    let path = hiscores_path();

    let ip_address = std::net::Ipv4Addr::new(127, 0, 0, 1);
    let socket_address = std::net::SocketAddrV4::new(ip_address, port);

    let server = Server::http(socket_address).unwrap();

    let handler = ScoresHandler::new(path);

    server.handle(handler).unwrap();
}

fn read_hiscores(file: &mut File) -> Vec<Score> {
    let mut hiscores = String::new();
    file.read_to_string(&mut hiscores).expect(
        "Hiscores file is invalid",
    );
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
    let mut path = std::env::home_dir().unwrap();
    path.push(TETRIS_CONF);

    let _ = DirBuilder::new().create(&path);

    path.push("hiscores.json");
    path
}