use super::PeekableSource;
pub use porus_macros::scanf_impl;

#[macro_export]
macro_rules! scanf {
    ($f:expr, $fmt:expr $(, $arg:expr)*) => (
        $crate::io::scanf::scanf_impl!($crate, $f, $fmt $(, $arg)*)
    )
}

pub trait Converter {
    fn write(&mut self, c: u8);
}


pub fn whitespace<S: PeekableSource<Item=u8>>(s: &mut S) -> &mut S {
    while let Some(&c) = PeekableSource::peek(s) {
        match c {
            b' ' | b'\t' ... b'\r' => { PeekableSource::consume(s); },
            _ => { break; },
        }
    }

    s
}

pub fn exact<S: PeekableSource<Item=u8>>(s: &mut S, c: u8) -> &mut S  {
    if let Some(&ch) = PeekableSource::peek(s) {
        if c == ch {
            PeekableSource::consume(s);
            return s;
        }
    }

    abort!("scan error");
}

pub fn character<'a, S: PeekableSource<Item=u8>, C: Converter>(s: &'a mut S, cv: &mut C) -> &'a mut S {
    if let Some(&c) = PeekableSource::peek(s) {
        Converter::write(cv, c);
        PeekableSource::consume(s);
        return s;
    }
    abort!("scan error");
}

fn is_digit(c: u8, base: u8) -> bool {
    let d =
        match c {
            b'0' ... b'9' => { c - b'0' },
            b'A' ... b'Z' => { c - b'A' + 10u8 },
            b'a' ... b'z' => { c - b'a' + 10u8 },
            _ => { return false; },
        };
    d < base
}

pub fn unsigned<'a, S: PeekableSource<Item=u8>, C: Converter>(s: &'a mut S, cv: &mut C, base: u8) -> &'a mut S {
    match PeekableSource::peek(s) {
        Some(&c) if is_digit(c, base) => {
            Converter::write(cv, c);
            PeekableSource::consume(s);

            while let Some(&c) = PeekableSource::peek(s) {
                if is_digit(c, base) {
                    Converter::write(cv, c);
                    PeekableSource::consume(s);
                } else {
                    break;
                }
            }

            s
        },
        _ => abort!("scan error"),
    }
}

pub fn signed<'a, S: PeekableSource<Item=u8>, C: Converter>(s: &'a mut S, cv: &mut C, base: u8) -> &'a mut S {
    match PeekableSource::peek(s) {
        Some(&b'-')  => {
            Converter::write(cv, b'-');
            PeekableSource::consume(s);
        },
        _ => {},
    }

    unsigned(s, cv, base)
}


pub struct Ignore;

impl Converter for Ignore {
    fn write(&mut self, _: u8) {
    }
}

pub trait CharPattern {
    type Converter: Converter;

    fn converter(self) -> Self::Converter;
}

pub trait UnsignedPattern {
    type Converter: Converter;

    fn converter(self, base: u8) -> Self::Converter;
}

pub trait SignedPattern {
    type Converter: Converter;

    fn converter(self, base: u8) -> Self::Converter;
}
