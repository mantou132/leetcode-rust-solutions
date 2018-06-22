use core::ops::{Div, Rem, Neg};
use core::convert::TryInto;
use super::super::iter::Iterator;
use super::Sink;

pub fn fwrite<'a, S : 'a + Sink, F : FnMut(&'a mut S)>(sink: &'a mut S, f: &mut F) {
    f(sink)
}

#[cfg(not(doc))]
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

#[cfg(doc)]
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
    fn write<S: Sink>(self, s: &mut S, radix: u8, width: usize);
}

fn to_char(d: u8) -> u8 {
    match d {
        0 ... 9 => { b'0' + d },
        10 ... 35 => { b'A' + d - 10 },
        _ => { panic!() },
    }
}

fn write_unsigned<S: Sink, T: Copy + Default + PartialOrd + Div<Output=T> + Rem<Output=T> + TryInto<u8>>(s: &mut S, mut x: T, radix: T, width: usize) {
    let mut buf = [b'0';40];
    let mut i = 39;

    while x > Default::default() {
        buf[i] = to_char(TryInto::try_into(x % radix).ok().unwrap());
        i -= 1;
        x = x /radix;
    }

    i = Ord::min(i+1, 40-width);
    fwrite_str(s, &buf[i..]);
}

fn write_signed<S: Sink, T: Copy + Default + PartialOrd + Neg<Output=T> + Div<Output=T> + Rem<Output=T> + TryInto<u8>>(s: &mut S, x: T, radix: T, width: usize) {
    if x < -x {
        Sink::write(s, b'-');
        write_unsigned(s, -x, radix, width);
    } else {
        write_unsigned(s, x, radix, width);
    }
}

macro_rules! unsigned {
    ($t:ty) => (
        impl Int for $t {
            fn write<S: Sink>(self, s: &mut S, radix: u8, width: usize) {
                write_unsigned(s, self, radix as _, width)
            }
        }
    )
}

macro_rules! signed {
    ($t:ty) => (
        impl Int for $t {
            fn write<S: Sink>(self, s: &mut S, radix: u8, width: usize) {
                write_signed(s, self, radix as _, width)
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


pub trait Float {
    fn write<S: Sink>(self, s: &mut S, prec: i32);
}

use core::intrinsics::powif64;

impl Float for f64 {

    fn write<S: Sink>(mut self, s: &mut S, prec: i32) {
        if self.is_finite() {
            #[cfg(any(all(debug_assertions, not(test)), local))]
            {
                fwrite_str(s, b"\x1bXf.");
                write_unsigned(s, prec, 10, 1);
                fwrite_str(s, b"\x1b\\");
            }

            if self.is_sign_negative() {
                Sink::write(s, b'-');
                self = -self;
            }

            self *= unsafe { powif64(10.0, prec) };
            let i = self as u64;
            let m = 10u64.pow(prec as _);

            if self <= 9007199254740992.0 {
                write_unsigned(s, i / m, 10, 1);
                Sink::write(s, b'.');
                write_unsigned(s, i % m, 10, prec as _);
                return;
            }
        }

        panic!("floating number out of range");
    }
}
