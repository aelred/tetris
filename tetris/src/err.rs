use std;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<Error>>;
