use core::ops::{Div, Rem, Neg};
use core::convert::TryInto;
use super::super::iter::Iterator;
use super::Sink;

pub fn fwrite<'a, S : 'a + Sink, F : FnMut(&'a mut S)>(sink: &'a mut S, mut f: F) {
    f(sink)
}

#[cfg(feature="build")]
pub fn join<'a, S : 'a + Sink, Sep : FnMut(&'a mut S), F : FnMut(&'a mut S), I : Iterator<Item=F>>(mut sep: Sep, mut it: I) -> impl FnMut(&'a mut S) {
    move |s: &'a mut S| {
        let iter = &mut it;

        match Iterator::next(iter) {
            None => { return; }
            Some(mut f) => { f(s); }
        }

        for mut f in iter {
            sep(s);
            f(s);
        }
    }
}

#[cfg(not(feature="build"))]
pub fn join<'a, S : 'a + Sink, Sep : FnMut(&'a mut S), F : FnMut(&'a mut S), I : Iterator<Item=F>>(mut sep: Sep, mut it: I) -> impl FnMut(&'a mut S) {
    move |s: &'a mut S| {
        panic!();
    }
}

pub fn fwrite_str<S: Sink, T: AsRef<[u8]>>(s: &mut S, t: T) {
    for c in AsRef::<[u8]>::as_ref(&t) {
        Sink::write(s, *c);
    }
}

pub trait String {
    fn write<S: Sink>(self, s: &mut S);
}

impl<'a> String for &'a str {
    fn write<S: Sink>(self, s: &mut S) {
        fwrite_str(s, self);
    }
}

pub trait Int {
    fn write<S: Sink>(self, s: &mut S, radix: u8);
}

fn to_char(d: u8) -> u8 {
    match d {
        0 ... 9 => { b'0' + d },
        10 ... 35 => { b'A' + d - 10 },
        _ => { panic!() },
    }
}

fn write_unsigned_aux<S: Sink, T : Copy + Default + PartialEq + Div<Output=T> + Rem<Output=T> + TryInto<u8>>(s: &mut S, x: T, radix: T) {
    if x != Default::default() {
        write_unsigned_aux(s, x / radix, radix);
        let d = TryInto::try_into(x % radix).ok().unwrap();
        Sink::write(s, to_char(d));
    }
}

fn write_unsigned<S: Sink, T: Copy + Default + PartialEq + Div<Output=T> + Rem<Output=T> + TryInto<u8>>(s: &mut S, x: T, radix: T) {
    if x == Default::default() {
        Sink::write(s, b'0');
    } else {
        write_unsigned_aux(s, x, radix)
    }
}

fn write_signed<S: Sink, T: Copy + Default + PartialOrd + Neg<Output=T> + Div<Output=T> + Rem<Output=T> + TryInto<u8>>(s: &mut S, x: T, radix: T) {
    if x < Default::default() {
        Sink::write(s, b'-');
        write_unsigned(s, -x, radix);
    } else {
        write_unsigned(s, x, radix);
    }
}

macro_rules! unsigned {
    ($t:ty) => (
        impl Int for $t {
            fn write<S: Sink>(self, s: &mut S, radix: u8) {
                write_unsigned(s, self, radix as _)
            }
        }
    )
}

macro_rules! signed {
    ($t:ty) => (
        impl Int for $t {
            fn write<S: Sink>(self, s: &mut S, radix: u8) {
                write_signed(s, self, radix as _)
            }
        }
    )
}

unsigned!(u8);
unsigned!(u16);
unsigned!(u32);
unsigned!(u64);
unsigned!(u128);
unsigned!(usize);

signed!(i8);
signed!(i16);
signed!(i32);
signed!(i64);
signed!(i128);
signed!(isize);
