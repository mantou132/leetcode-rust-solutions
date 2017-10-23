use core::cmp::PartialOrd;
use core::ops::{Add, Mul, Div, Rem, Neg, Not};
use core::marker::{Sized, Copy};
use super::super::traits::{InputStream, OutputStream};


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

pub fn read_unsigned<T: Unsigned, Stream: InputStream>(stream: &mut Stream) -> T {
    let mut x : T = from_char(0u8);

    loop {
        match stream.peek() {
            Some(&c) if (c >= b'0') && (c <= b'9') =>
                x = x * from_char(10u8) + from_char(c - b'0'),
            _ =>
                break,
        }

        stream.consume()
    }

    x
}

fn write_unsigned_aux<T: Unsigned, Stream: OutputStream>(stream: &mut Stream, u: T) {
    if u != from_char(0u8) {
        write_unsigned_aux(stream, u / from_char(10u8));
        OutputStream::write_char(stream, b'0' + to_char(u % from_char(10u8)));
    }
}

pub fn write_unsigned<T: Unsigned, Stream: OutputStream>(stream: &mut Stream, u: T) {
    if u == from_char(0u8) {
        OutputStream::write_char(stream, b'0');
    } else {
        write_unsigned_aux(stream, u);
    }
}


pub trait Signed : Sized + Copy + Add<Self,Output=Self> + Mul<Self,Output=Self> + Neg<Output=Self> + PartialOrd<Self> + FromChar {
    type Unsigned : Unsigned;

    fn to_unsigned(x: Self) -> Self::Unsigned;
}

fn to_unsigned<T: Signed>(x: T) -> T::Unsigned {
    Signed::to_unsigned(x)
}

pub fn read_signed<T: Signed, Stream: InputStream>(stream: &mut Stream) -> T {
    let mut x : T = from_char(0u8);
    let mut s : T = from_char(1u8);

    match stream.peek() {
        Some(&b'-') => {
            s = -s;
            stream.consume()
        },
        _ => {
        },
    }

    loop {
        match stream.peek() {
            Some(&c) if (c >= b'0') && (c <= b'9') =>
                x = x * from_char(10u8) + from_char(c - b'0'),
            _ =>
                break,
        }

        stream.consume()
    }

    x*s
}


fn write_signed_aux<T: Unsigned, Stream: OutputStream>(stream: &mut Stream, u: T, neg: bool) {
    if u == from_char(0u8) {
        if neg {
            OutputStream::write_char(stream, b'-');
        }
    } else {
        write_signed_aux(stream, u / from_char(10), neg);
        OutputStream::write_char(stream, b'0' + to_char(u % from_char(10)));
    }
}


pub fn write_signed<T: Signed, Stream: OutputStream>(stream: &mut Stream, i: T) {
    if i == from_char(0u8) {
        OutputStream::write_char(stream, b'0');
    } else {
        let neg = i < from_char(0u8);
        let mut u = to_unsigned(i);

        if neg {
            u = !u + from_char(1u8);
        }

        write_signed_aux(stream, u, neg);
    }
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
