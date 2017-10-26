use super::super::compat::prelude::*;
use super::Sink;
use super::num::{write_unsigned, write_signed};

pub trait WriteArg<T> {
    fn write<S: Sink<Item=T>>(sink: &mut S, Self) -> Result<(), S::Error>;
}

pub fn write<T, W: WriteArg<T>, S: Sink<Item=T>>(sink: &mut S, x: W) -> Result<(), S::Error> {
    WriteArg::write(sink, x)
}


impl WriteArg<u8> for u8 {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_unsigned(sink, x)
    }
}

impl WriteArg<u8> for u16 {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_unsigned(sink, x)
    }
}

impl WriteArg<u8> for u32 {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_unsigned(sink, x)
    }
}

impl WriteArg<u8> for u64 {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_unsigned(sink, x)
    }
}

impl WriteArg<u8> for usize {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_unsigned(sink, x)
    }
}



impl WriteArg<u8> for i8 {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_signed(sink, x)
    }
}

impl WriteArg<u8> for i16 {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_signed(sink, x)
    }
}

impl WriteArg<u8> for i32 {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_signed(sink, x)
    }
}

impl WriteArg<u8> for i64 {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_signed(sink, x)
    }
}

impl WriteArg<u8> for isize {
    fn write<S: Sink<Item=u8>>(sink: &mut S, x: Self) -> Result<(), S::Error> {
        write_signed(sink, x)
    }
}


impl<'a, T: Copy> WriteArg<T> for &'a[T] {
    fn write<S: Sink<Item=T>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        for c in x {
            Sink::write(s, *c)?;
        }
        Ok(())
    }
}

impl<'a> WriteArg<u8> for &'a str {
    fn write<S: Sink<Item=u8>>(s: &mut S, x: Self) -> Result<(), S::Error> {
        write(s, x.as_bytes())
    }
}
