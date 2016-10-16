use core::cmp::PartialOrd;
use core::ops::{Add, Mul, Div, Rem, Neg, Not};
use core::marker::{Sized, Copy};
use super::traits::{InputStream, OutputStream};

pub mod file;


trait FromU8 {
    fn from_u8(x: u8) -> Self;
}

trait ToU8 {
    fn to_u8(x: Self) -> u8;
}

trait Unsigned : Sized + Copy + Add<Self,Output=Self> + Mul<Self,Output=Self> + Div<Self,Output=Self> + Rem<Self,Output=Self> + Not<Output=Self> + PartialOrd<Self> + FromU8 + ToU8 {
}

trait Signed : Sized + Copy + Add<Self,Output=Self> + Mul<Self,Output=Self> + Neg<Output=Self> + PartialOrd<Self> + FromU8 {
    type Unsigned : Unsigned;

    fn to_unsigned(x: Self) -> Self::Unsigned;
}


pub fn eof<Stream: InputStream>(stream: &mut Stream) -> bool {
    match stream.peek() {
        None => true,
        _ => false,
    }
}

pub fn ignore<Stream: InputStream, Fun: Fn(u8) -> bool>(stream: &mut Stream, ignore: Fun) -> &mut Stream {
    loop {
        match stream.peek() {
            Some(&c) if ignore(c) => (),
            _ => break,
        }

        stream.consume()
    }

    stream
}


fn read_unsigned<T: Unsigned, Stream: InputStream>(stream: &mut Stream) -> T {
    let mut x = T::from_u8(0u8);

    loop {
        match stream.peek() {
            Some(&c) if (c >= b'0') && (c <= b'9') =>
                x = x * T::from_u8(10u8) + T::from_u8(c - b'0'),
            _ =>
                break,
        }

        stream.consume()
    }

    x
}


fn read_signed<T: Signed, Stream: InputStream>(stream: &mut Stream) -> T {
    let mut x = T::from_u8(0u8);
    let mut s = T::from_u8(1u8);

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
                x = x * T::from_u8(10u8) + T::from_u8(c - b'0'),
            _ =>
                break,
        }

        stream.consume()
    }

    x*s
}


fn write_unsigned_aux<T: Unsigned, Stream: OutputStream>(stream: &mut Stream, u: T) {
    if u != T::from_u8(0u8) {
        write_unsigned_aux(stream, u / T::from_u8(10));
        stream.write(b'0' + T::to_u8(u % T::from_u8(10)));
    }
}


fn write_unsigned<T: Unsigned, Stream: OutputStream>(stream: &mut Stream, u: T) {
    if u == T::from_u8(0u8) {
        stream.write(b'0');
    } else {
        write_unsigned_aux(stream, u);
    }
}


fn write_signed_aux<T: Unsigned, Stream: OutputStream>(stream: &mut Stream, u: T, neg: bool) {
    if u == T::from_u8(0u8) {
        if neg {
            stream.write(b'-');
        }
    } else {
        write_signed_aux(stream, u / T::from_u8(10), neg);
        stream.write(b'0' + T::to_u8(u % T::from_u8(10)));
    }
}


fn write_signed<T: Signed, Stream: OutputStream>(stream: &mut Stream, i: T) {
    if i == T::from_u8(0u8) {
        stream.write(b'0');
    } else {
        let neg = i < T::from_u8(0u8);
        let mut u = T::to_unsigned(i);

        if neg {
            u = T::Unsigned::from_u8(1u8) + !u;
        }

        write_signed_aux(stream, u, neg);
    }
}


pub fn read_u8<Stream: InputStream>(stream: &mut Stream) -> u8 {
    read_unsigned::<u8, Stream>(stream)
}

pub fn read_u16<Stream: InputStream>(stream: &mut Stream) -> u16 {
    read_unsigned::<u16, Stream>(stream)
}

pub fn read_u32<Stream: InputStream>(stream: &mut Stream) -> u32 {
    read_unsigned::<u32, Stream>(stream)
}

pub fn read_u64<Stream: InputStream>(stream: &mut Stream) -> u64 {
    read_unsigned::<u64, Stream>(stream)
}

pub fn read_usize<Stream: InputStream>(stream: &mut Stream) -> usize {
    read_unsigned::<usize, Stream>(stream)
}

pub fn read_i8<Stream: InputStream>(stream: &mut Stream) -> i8 {
    read_signed::<i8, Stream>(stream)
}

pub fn read_i16<Stream: InputStream>(stream: &mut Stream) -> i16 {
    read_signed::<i16, Stream>(stream)
}

pub fn read_i32<Stream: InputStream>(stream: &mut Stream) -> i32 {
    read_signed::<i32, Stream>(stream)
}

pub fn read_i64<Stream: InputStream>(stream: &mut Stream) -> i64 {
    read_signed::<i64, Stream>(stream)
}

pub fn read_isize<Stream: InputStream>(stream: &mut Stream) -> isize {
    read_signed::<isize, Stream>(stream)
}

pub fn write_u8<Stream: OutputStream>(stream: &mut Stream, x: u8) {
    write_unsigned::<u8, Stream>(stream, x)
}

pub fn write_u16<Stream: OutputStream>(stream: &mut Stream, x: u16) {
    write_unsigned::<u16, Stream>(stream, x)
}

pub fn write_u32<Stream: OutputStream>(stream: &mut Stream, x: u32) {
    write_unsigned::<u32, Stream>(stream, x)
}

pub fn write_u64<Stream: OutputStream>(stream: &mut Stream, x: u64) {
    write_unsigned::<u64, Stream>(stream, x)
}

pub fn write_usize<Stream: OutputStream>(stream: &mut Stream, x: usize) {
    write_unsigned::<usize, Stream>(stream, x)
}

pub fn write_i8<Stream: OutputStream>(stream: &mut Stream, x: i8) {
    write_signed::<i8, Stream>(stream, x)
}

pub fn write_i16<Stream: OutputStream>(stream: &mut Stream, x: i16) {
    write_signed::<i16, Stream>(stream, x)
}

pub fn write_i32<Stream: OutputStream>(stream: &mut Stream, x: i32) {
    write_signed::<i32, Stream>(stream, x)
}

pub fn write_i64<Stream: OutputStream>(stream: &mut Stream, x: i64) {
    write_signed::<i64, Stream>(stream, x)
}

pub fn write_isize<Stream: OutputStream>(stream: &mut Stream, x: isize) {
    write_signed::<isize, Stream>(stream, x)
}

pub fn write_s<Stream: OutputStream>(stream: &mut Stream, x: &[u8]) {
    for c in x {
        stream.write(*c);
    }
}

use super::string::String;
pub use super::string::{read_string, read_string_until};

#[cfg_attr(not(debug_assertions), inline(always))]
pub fn write_string<Stream: OutputStream>(stream: &mut Stream, x: &String) {
    write_s(stream, x);
}


impl FromU8 for u8 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl FromU8 for u16 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl FromU8 for u32 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl FromU8 for u64 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl FromU8 for usize {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl FromU8 for i8 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl FromU8 for i16 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl FromU8 for i32 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl FromU8 for i64 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl FromU8 for isize {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn from_u8(x: u8) -> Self { x as Self }
}

impl ToU8 for u8 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_u8(x: Self) -> u8 { x as u8 }
}

impl ToU8 for u16 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_u8(x: Self) -> u8 { x as u8 }
}

impl ToU8 for u32 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_u8(x: Self) -> u8 { x as u8 }
}

impl ToU8 for u64 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_u8(x: Self) -> u8 { x as u8 }
}

impl ToU8 for usize {
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_u8(x: Self) -> u8 { x as u8 }
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

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}

impl Signed for i16 {
    type Unsigned = u16;

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}

impl Signed for i32 {
    type Unsigned = u32;

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}

impl Signed for i64 {
    type Unsigned = u64;

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}

impl Signed for isize {
    type Unsigned = usize;

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn to_unsigned(x: Self) -> Self::Unsigned { x as Self::Unsigned }
}
