use core::ops::{Add, Mul, Neg};
use core::convert::TryFrom;
use super::{Source, PeekableSource};


pub trait Consumer {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>) -> bool;
}

pub fn fread<I : Source, C: Consumer>(s: &mut PeekableSource<I>, c: C) -> bool {
    Consumer::consume(c, s)
}

pub struct Whitespace;

impl Consumer for Whitespace {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>) -> bool {
        while let Some(&c) = s.peek() {
            match c {
                b' ' | b'\t' ... b'\r' => { s.consume(); },
                _ => { break; },
            }
        }
        true
    }
}

pub struct Char<'a>(pub &'a mut u8);

impl<'a> Consumer for Char<'a> {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>) -> bool {
        match s.peek() {
            None => false,
            Some(&c) => {
                *(self.0) = c;
                s.consume();
                true
            }
        }
    }
}


pub struct Int<'a, T : 'a>(&'a mut T, u8);

pub fn bin<'a, T : 'a>(i: &'a mut T) -> Int<'a, T> {
    Int(i, 2)
}

pub fn oct<'a, T : 'a>(i: &'a mut T) -> Int<'a, T> {
    Int(i, 8)
}

pub fn hex<'a, T : 'a>(i: &'a mut T) -> Int<'a, T> {
    Int(i, 16)
}

fn read_digit<I : Source>(s: &mut PeekableSource<I>, radix: u8) -> Option<u8> {
    let c =
        match s.peek() {
            None => { return None; },
            Some(&x) => { x },
        };

    let d =
        match c {
            b'0' ... b'9' => { c - b'0' },
            b'A' ... b'Z' => { c - b'A' + 10u8 },
            b'a' ... b'z' => { c - b'a' + 10u8 },
            _ => { return None; },
        };

    if d >= radix {
        return None;
    }

    s.consume();
    Some(d)
}

fn read_unsigned<I : Source, T : Copy + Default + Add<Output=T> + Mul<Output=T> + TryFrom<u8>>(s: &mut PeekableSource<I>, radix: u8) -> Option<T> {
    match read_digit(s, radix) {
        None => None,
        Some(d) => {
            let mut x : T = TryFrom::try_from(d).ok().unwrap();

            while let Some(d) = read_digit(s, radix) {
                x = x * TryFrom::try_from(10).ok().unwrap() + TryFrom::try_from(d).ok().unwrap();
            }

            Some(x)
        }
    }
}

fn read_signed<I : Source, T : Copy + Default + Add<Output=T> + Mul<Output=T> + TryFrom<u8> + Neg<Output=T>>(s: &mut PeekableSource<I>, radix: u8) -> Option<T> {
    match s.peek() {
        None => None,
        Some(&b'-') => {
            s.consume();
            let x : T = read_unsigned(s, radix).unwrap();
            Some(-x)
        },
        Some(_) => {
            read_unsigned(s, radix)
        }
    }
}


impl<'a, T : 'a + Copy + Default + Add<Output=T> + Mul<Output=T> + TryFrom<u8>> Consumer for Int<'a, T> {
    default fn consume<I : Source>(self, s: &mut PeekableSource<I>) -> bool {
        match read_unsigned(s, self.1) {
            None => false,
            Some(v) => {
                *self.0 = v;
                true
            }
        }
    }
}

impl<'a, T : 'a + Copy + Default + Add<Output=T> + Mul<Output=T> + TryFrom<u8> + Neg<Output=T>> Consumer for Int<'a, T> {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>) -> bool {
        match read_signed(s, self.1) {
            None => false,
            Some(v) => {
                *self.0 = v;
                true
            }
        }
    }
}

macro_rules! int {
    ($t:ty) => (
        impl<'a> Consumer for &'a mut $t {
            fn consume<I : Source>(self, s: &mut PeekableSource<I>) -> bool {
                Consumer::consume(Int(self, 10), s)
            }
        }
    )
}

int!(u8);
int!(u16);
int!(u32);
int!(u64);
int!(u128);
int!(usize);

int!(i8);
int!(i16);
int!(i32);
int!(i64);
int!(i128);
int!(isize);

use core::intrinsics::powif64;

impl<'a> Consumer for &'a mut f64 {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>) -> bool {
        let sign : f64 =
            if let Some(&b'-') = s.peek() {
                s.consume();
                -1.0
            } else {
                1.0
            };

        let mut int : u64 = 0;
        fread(s, &mut int);

        let mut exp : i32 = 0;

        if let Some(&b'.') = s.peek() {
            s.consume();

            while let Some(d) = read_digit(s, 10) {
                int = int * 10 + (d as u64);
                exp -= 1;
            }
        }

        if let Some(&b'e') = s.peek() {
            s.consume();
            let mut e : i32 = 0;
            fread(s, &mut e);
            exp += e;
        }

        *self = sign * unsafe { powif64(10.0, exp) } * (int as f64);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::super::slice::SliceSource;
    use super::{fread, hex, Whitespace};

    #[test]
    fn test_whitespace() {
        let source = &mut SliceSource::new(b"   ");
        fread(source, Whitespace);
        assert!(source.eof());
    }

    #[test]
    fn test_unsigned_match() {
        let source = &mut SliceSource::new(b"a");
        let mut x = 0usize;
        fread(source, hex(&mut x));
        assert!(x == 0xa);
    }

    #[test]
    fn test_unsigned_mismatch() {
        let source = &mut SliceSource::new(b"g");
        let mut x = 0usize;
        assert!(!fread(source, hex(&mut x)));
    }

    #[test]
    fn test_unsigned_mismatch_empty() {
        let source = &mut SliceSource::new(b"");
        let mut x = 0usize;
        assert!(!fread(source, hex(&mut x)));
    }

    #[test]
    fn test_signed_match() {
        let source = &mut SliceSource::new(b"-123");
        let mut x = 0isize;
        fread(source, &mut x);
        assert!(x == -123);
    }

    #[test]
    #[should_panic]
    fn test_signed_mismatch() {
        let source = &mut SliceSource::new(b"-g");
        let mut x = 0isize;
        fread(source, &mut x);
    }

    #[test]
    fn test_signed_mismatch_empty() {
        let source = &mut SliceSource::new(b"");
        let mut x = 0isize;
        assert!(!fread(source, &mut x));
    }

    #[test]
    #[should_panic]
    fn test_signed_mismatch_sign() {
        let source = &mut SliceSource::new(b"-");
        let mut x = 0isize;
        fread(source, &mut x);
    }
}
