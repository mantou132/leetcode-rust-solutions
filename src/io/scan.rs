use super::super::compat::prelude::*;
use super::read::ReadArg;
use std::error::Error;


pub trait Scanner {
    type Item;
    type Error: Error;

    fn scan<R: ReadArg<Self::Item>>(&mut self, r: R) -> Result<(), Self::Error>;
}

pub trait ScanArg<T> {
    fn scan<S: Scanner<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error>;
}

pub fn scan<T, R: ScanArg<T>, S: Scanner<Item=T>>(scanner: &mut S, x: R) -> Result<(), S::Error> {
    ScanArg::scan(scanner, x)
}

impl<T> ScanArg<T> for () {
    fn scan<S: Scanner<Item=T>>(_: &mut S, _: Self) -> Result<(), S::Error> {
        Ok(())
    }
}

impl<T,A> ScanArg<T> for (A,)
    where A: ReadArg<T> {
    fn scan<S: Scanner<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        Scanner::scan(s, x.0)?;
        Ok(())
    }
}

impl<T,A,B> ScanArg<T> for (A,B)
    where A: ReadArg<T>,
          B: ReadArg<T> {
    fn scan<S: Scanner<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        Scanner::scan(s, x.0)?;
        Scanner::scan(s, x.1)?;
        Ok(())
    }
}

impl<T,A,B,C> ScanArg<T> for (A,B,C)
    where A: ReadArg<T>,
          B: ReadArg<T>,
          C: ReadArg<T> {
    fn scan<S: Scanner<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        Scanner::scan(s, x.0)?;
        Scanner::scan(s, x.1)?;
        Scanner::scan(s, x.2)?;
        Ok(())
    }
}

impl<T,A,B,C,D> ScanArg<T> for (A,B,C,D)
    where A: ReadArg<T>,
          B: ReadArg<T>,
          C: ReadArg<T>,
          D: ReadArg<T> {
    fn scan<S: Scanner<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        Scanner::scan(s, x.0)?;
        Scanner::scan(s, x.1)?;
        Scanner::scan(s, x.2)?;
        Scanner::scan(s, x.3)?;
        Ok(())
    }
}
