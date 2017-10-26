use super::super::compat::prelude::*;
use std::ops::{Add, Mul, Div, Rem, Neg, Not};
use super::{PeekableSource, ScanError, Sink};

pub trait FromChar {
    fn from_char(u8) -> Self;
}

fn from_char<T : FromChar>(n: u8) -> T {
    return FromChar::from_char(n)
}

pub trait ToChar {
    fn to_char(self) -> u8;
}

fn to_char<T : ToChar>(n: T) -> u8 {
    return ToChar::to_char(n)
}

pub trait Unsigned : Sized + Copy + Add<Self,Output=Self> + Mul<Self,Output=Self> + Div<Self,Output=Self> + Rem<Self,Output=Self> + Not<Output=Self> + PartialOrd<Self> + FromChar + ToChar {
}

pub trait Signed : Sized + Copy + Add<Self,Output=Self> + Mul<Self,Output=Self> + Neg<Output=Self> + PartialOrd<Self> + FromChar {
    type Unsigned : Unsigned;

    fn to_unsigned(x: Self) -> Self::Unsigned;
}

fn to_unsigned<T: Signed>(x: T) -> T::Unsigned {
    Signed::to_unsigned(x)
}


pub fn read_unsigned<T: Unsigned, S: PeekableSource<Item=u8>>(source: &mut S, t: &mut T) -> Result<(), ScanError<S::Error>> {
    match PeekableSource::peek(source) {
        Some(&c) if (c >= b'0') && (c <= b'9') => {
            PeekableSource::consume(source)?;

            let mut x : T = from_char(c - b'0');
            loop {
                match PeekableSource::peek(source) {
                    Some(&c) if (c >= b'0') && (c <= b'9') =>
                        x = x * from_char(10u8) + from_char(c - b'0'),
                    _ =>
                        break,
                }

                PeekableSource::consume(source)?
            }

            *t = x;
            Ok(())
        }
        None => Err(ScanError::EOF),
        _ => Err(ScanError::BadInput)
    }
}

fn write_unsigned_aux<T: Unsigned, S: Sink<Item=u8>>(sink: &mut S, u: T) -> Result<(),S::Error> {
    if u != from_char(0u8) {
        write_unsigned_aux(sink, u / from_char(10u8))?;
        Sink::write(sink, b'0' + to_char(u % from_char(10u8)))?;
    }
    Ok(())
}

pub fn write_unsigned<T: Unsigned, S: Sink<Item=u8>>(sink: &mut S, u: T) -> Result<(),S::Error> {
    if u == from_char(0u8) {
        Sink::write(sink, b'0')?;
    } else {
        write_unsigned_aux(sink, u)?;
    }
    Ok(())
}


pub fn read_signed<T: Signed, S: PeekableSource<Item=u8>>(source: &mut S, t: &mut T) -> Result<(),ScanError<S::Error>> {
    let mut s : T = from_char(1u8);

    match PeekableSource::peek(source) {
        Some(&b'-') => {
            s = -s;
            PeekableSource::consume(source)?
        },
        _ => {
        },
    }

    match PeekableSource::peek(source) {
        Some(&c) if (c >= b'0') && (c <= b'9') => {
            PeekableSource::consume(source)?;

            let mut x : T = from_char(c - b'0');
            loop {
                match PeekableSource::peek(source) {
                    Some(&c) if (c >= b'0') && (c <= b'9') =>
                        x = x * from_char(10u8) + from_char(c - b'0'),
                    _ =>
                        break,
                }

                PeekableSource::consume(source)?
            }

            *t = x*s;
            Ok(())
        }
        None if (s == from_char(1u8)) => Err(ScanError::EOF),
        _ => Err(ScanError::BadInput)
    }
}


fn write_signed_aux<T: Unsigned, S: Sink<Item=u8>>(sink: &mut S, u: T, neg: bool) -> Result<(),S::Error> {
    if u == from_char(0u8) {
        if neg {
            Sink::write(sink, b'-')?;
        }
    } else {
        write_signed_aux(sink, u / from_char(10), neg)?;
        Sink::write(sink, b'0' + to_char(u % from_char(10)))?;
    }
    Ok(())
}


pub fn write_signed<T: Signed, S: Sink<Item=u8>>(sink: &mut S, i: T) -> Result<(),S::Error> {
    if i == from_char(0u8) {
        Sink::write(sink, b'0')?;
    } else {
        let neg = i < from_char(0u8);
        let mut u = to_unsigned(i);

        if neg {
            u = !u + from_char(1u8);
        }

        write_signed_aux(sink, u, neg)?;
    }
    Ok(())
}


impl FromChar for u8 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for u16 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for u32 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for u64 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for usize {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for i8 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for i16 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for i32 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for i64 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for isize {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}


impl ToChar for u8 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for u16 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for u32 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for u64 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for usize {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for i8 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for i16 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for i32 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for i64 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for isize {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl Unsigned for u8 {
}

impl Unsigned for u16 {
}

impl Unsigned for u32 {
}

impl Unsigned for u64 {
}

impl Unsigned for usize {
}

impl Signed for i8 {
    type Unsigned = u8;

    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}

impl Signed for i16 {
    type Unsigned = u16;

    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}

impl Signed for i32 {
    type Unsigned = u32;

    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}

impl Signed for i64 {
    type Unsigned = u64;

    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}

impl Signed for isize {
    type Unsigned = usize;

    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}
