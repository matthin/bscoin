extern crate serde_json;

use std::io;

#[derive(Debug)]
pub enum BsError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for BsError {
    fn from(err: io::Error) -> BsError {
        BsError::Io(err)
    }
}

impl From<serde_json::Error> for BsError {
    fn from(err: serde_json::Error) -> BsError {
        BsError::Json(err)
    }
}
