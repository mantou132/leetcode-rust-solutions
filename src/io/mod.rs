use super::compat::prelude::*;
use std::error::Error;
use std::fmt;
use std::convert::From;

pub trait Source {
    type Item;
    type Error : Error;

    fn read(&mut self) -> Result<Option<Self::Item>,Self::Error>;
}

pub trait PeekableSource {
    type Item;
    type Error : Error;

    fn peek(&mut self) -> Option<&Self::Item>;
    fn consume(&mut self) -> Result<(), Self::Error>;

    fn eof(&mut self) -> bool {
        match self.peek() {
            None => true,
            _ => false,
        }
    }
}

pub trait Sink {
    type Item;
    type Error: Error;

    fn write(&mut self, c: Self::Item) -> Result<(),Self::Error>;
}


#[derive(Debug)]
pub enum ScanError<E: Error> {
    EOF,
    BadInput,
    Error(E)
}

impl<E: Error> fmt::Display for ScanError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScanError::EOF => write!(f, "EOF"),
            ScanError::BadInput => write!(f, "BadInput"),
            ScanError::Error(ref e) => write!(f, "Error({})", e),
        }
    }
}

impl<E: Error> Error for ScanError<E> {
    fn description(&self) -> &str {
        match *self {
            ScanError::EOF => "EOF",
            ScanError::BadInput => "BadInput",
            ScanError::Error(ref e) => Error::description(e),
        }
    }

    fn cause<'a>(&'a self) -> Option<&'a Error> {
        match *self {
            ScanError::EOF => None,
            ScanError::BadInput => None,
            ScanError::Error(ref e) => Some(e),
        }
    }
}

impl<E: Error> From<E> for ScanError<E> {
    fn from(error: E) -> Self {
        ScanError::Error(error)
    }
}

mod peek;
mod delimit;

mod file;

mod num;

mod read;
mod scan;
mod write;
mod print;

pub use self::read::read;
pub use self::scan::scan;
pub use self::write::write;
pub use self::print::print;

mod stdio;
pub use self::stdio::{stdin, stdout};


pub fn ignore<S: PeekableSource, Fun: Fn(&S::Item) -> bool>(s: &mut S, ignore: Fun) -> Result<(), ScanError<S::Error>> {
    loop {
        match PeekableSource::peek(s) {
            Some(c) if ignore(c) => (),
            _ => break,
        }

        PeekableSource::consume(s)?
    }
    Ok(())
}
