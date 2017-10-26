use super::super::compat::prelude::*;
use super::{PeekableSource, ScanError};
use super::scan::Scanner;
use super::read::{read, ReadArg};


pub struct DelimitedScanner<S: PeekableSource, Fun: Fn(&mut S) -> Result<(),ScanError<S::Error>>> {
    source: S,
    skip: Fun,
}

impl<S: PeekableSource,Fun: Fn(&mut S) -> Result<(),ScanError<S::Error>>> DelimitedScanner<S,Fun> {
    pub fn new(source: S, fun: Fun) -> Self {
        DelimitedScanner {
            source: source,
            skip: fun,
        }
    }
}

impl<S: PeekableSource, Fun: Fn(&mut S) -> Result<(),ScanError<S::Error>>> PeekableSource for DelimitedScanner<S, Fun> {
    type Item = S::Item;
    type Error = S::Error;

    fn peek(&mut self) -> Option<&Self::Item> {
        PeekableSource::peek(&mut self.source)
    }

    fn consume(&mut self) -> Result<(), Self::Error> {
        PeekableSource::consume(&mut self.source)
    }

    fn eof(&mut self) -> bool {
        PeekableSource::eof(&mut self.source)
    }
}

impl<S: PeekableSource, Fun: Fn(&mut S) -> Result<(),ScanError<S::Error>>> Scanner for DelimitedScanner<S, Fun> {
    type Item = S::Item;
    type Error = ScanError<S::Error>;

    fn scan<R: ReadArg<Self::Item>>(&mut self, r: R) -> Result<(), Self::Error> {
        (self.skip)(&mut self.source)?;
        read(&mut self.source, r)
    }

}
