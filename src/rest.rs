use rustc_serialize::json;
use std::error::Error;
use score::Score;
use score::ScoreMessage;


const HI_SCORES_ENDPOINT: &'static str = "http://localhost:4444/scores";


pub fn get_hiscores() -> Result<Vec<Score>, Box<Error>> {
    let body = try!(get_raw_hiscores());
    let hiscores = try!(json::decode(&body));
    Ok(hiscores)
}

pub fn post_hiscore(score: &ScoreMessage) {
    let body = json::encode(score).unwrap();
    let response = post_raw_hiscores(body);

    if let Err(e) = response {
        println!("Failed to post hiscores: {}", e);
    }
}

#[cfg(not(target_os="emscripten"))]
fn get_raw_hiscores() -> Result<String, Box<Error>> {
    use hyper::client::Client;
    use std::io::Read;

    let client = Client::new();
    let mut body = String::new();
    let mut res = try!(client.get(HI_SCORES_ENDPOINT).send());
    try!(res.read_to_string(&mut body));
    Ok(body)
}

#[cfg(target_os="emscripten")]
fn get_raw_hiscores() -> Result<String, Box<Error>> {
    use emscripten::em;

    let script = format!(r#"(function() {{
        var req = new XMLHttpRequest();
        req.open("GET", "{}", false);
        req.send(null);
        return req.responseText;
    }}())"#,
                         HI_SCORES_ENDPOINT);

    Ok(em::run_script_string(&script))
}

#[cfg(not(target_os="emscripten"))]
fn post_raw_hiscores(score: String) -> Result<(), Box<Error>> {
    use hyper::client::Client;

    let client = Client::new();
    try!(client.post(HI_SCORES_ENDPOINT).body(score.as_bytes()).send());
    Ok(())
}

#[cfg(target_os="emscripten")]
fn post_raw_hiscores(score: String) -> Result<(), Box<Error>> {
    use emscripten::em;

    let script = format!(r#"(function() {{
        var req = new XMLHttpRequest();
        req.open("POST", "{}", false);
        req.send(JSON.stringify({}));
    }}())"#,
                         HI_SCORES_ENDPOINT,
                         score);

    em::run_script(&script);
    Ok(())
}
