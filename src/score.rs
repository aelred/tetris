#[derive(RustcDecodable, RustcEncodable)]
pub struct Score {
    pub value: u32,
    pub name: String,
}

impl Score {
    pub fn new(value: u32, name: String) -> Self {
        Score {
            value: value,
            name: name,
        }
    }
}
