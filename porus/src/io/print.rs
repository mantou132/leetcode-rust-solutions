use super::super::compat::prelude::*;
use super::Sink;
use super::write::{write, WriteArg};

pub trait PrintArg<T> {
    fn print<S: Sink<Item=T>>(sink: &mut S, Self) -> Result<(), S::Error>;
}

pub fn print<T, W: PrintArg<T>, S: Sink<Item=T>>(sink: &mut S, x: W) -> Result<(), S::Error> {
    PrintArg::print(sink, x)
}

impl<T> PrintArg<T> for () {
    fn print<S: Sink<Item=T>>(_: &mut S, _: Self) -> Result<(), S::Error> {
        Ok(())
    }
}

impl<T,A> PrintArg<T> for (A,)
    where A: WriteArg<T> {
    fn print<S: Sink<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        write(s, x.0)?;
        Ok(())
    }
}

impl<T,A,B> PrintArg<T> for (A,B)
    where A: WriteArg<T>,
          B: WriteArg<T> {
    fn print<S: Sink<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        write(s, x.0)?;
        write(s, x.1)?;
        Ok(())
    }
}

impl<T,A,B,C> PrintArg<T> for (A,B,C)
    where A: WriteArg<T>,
          B: WriteArg<T>,
          C: WriteArg<T> {
    fn print<S: Sink<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        write(s, x.0)?;
        write(s, x.1)?;
        write(s, x.2)?;
        Ok(())
    }
}

impl<T,A,B,C,D> PrintArg<T> for (A,B,C,D)
    where A: WriteArg<T>,
          B: WriteArg<T>,
          C: WriteArg<T>,
          D: WriteArg<T> {
    fn print<S: Sink<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        write(s, x.0)?;
        write(s, x.1)?;
        write(s, x.2)?;
        write(s, x.3)?;
        Ok(())
    }
}

impl<T,A,B,C,D,E> PrintArg<T> for (A,B,C,D,E)
    where A: WriteArg<T>,
          B: WriteArg<T>,
          C: WriteArg<T>,
          D: WriteArg<T>,
          E: WriteArg<T> {
    fn print<S: Sink<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        write(s, x.0)?;
        write(s, x.1)?;
        write(s, x.2)?;
        write(s, x.3)?;
        write(s, x.4)?;
        Ok(())
    }
}


impl<T,A,B,C,D,E,F> PrintArg<T> for (A,B,C,D,E,F)
    where A: WriteArg<T>,
          B: WriteArg<T>,
          C: WriteArg<T>,
          D: WriteArg<T>,
          E: WriteArg<T>,
          F: WriteArg<T> {
    fn print<S: Sink<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        write(s, x.0)?;
        write(s, x.1)?;
        write(s, x.2)?;
        write(s, x.3)?;
        write(s, x.4)?;
        write(s, x.5)?;
        Ok(())
    }
}
