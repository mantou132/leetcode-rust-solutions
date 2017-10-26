use std::convert::From;
use std::error;

#[derive(Debug)]
pub struct Error;

impl<T: error::Error> From<T> for Error {
    fn from(_: T) -> Self {
        Self{}
    }
}
