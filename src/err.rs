use std::error::Error;
use std;

pub type Result<T> = std::result::Result<T, Box<Error>>;