extern crate lib;
extern crate hyper;
extern crate hyperlocal;
extern crate rustc_serialize;

use lib::score::Score;

use rustc_serialize::json;

use hyper::Get;
use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use hyper::header::ContentType;
use hyperlocal::UnixSocketServer;

macro_rules! try_return(
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { println!("Error: {}", e); return; }
        }
    }}
);

fn scores(req: Request, mut res: Response) {
    match req.uri {
        AbsolutePath(ref path) => {
            match (&req.method, &path[..]) {
                (&Get, "/scores") => {
                    res.headers_mut().set(ContentType::json());
                    let scores = get_hiscores();
                    try_return!(res.send(json::encode(&scores).unwrap().as_bytes()));
                }
                _ => {
                    *res.status_mut() = hyper::NotFound;
                }
            }
        }
        _ => {}
    };

    return;
}

fn main() {
    let server = Server::http("localhost:4444").unwrap();
    let _ = server.handle(scores);
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
