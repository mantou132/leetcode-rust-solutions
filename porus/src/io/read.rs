use core::str::from_utf8_unchecked;
use core::num::ParseIntError;
use super::{Source, PeekableSource, Sink};
use super::slice::SliceSink;


pub trait Consumer {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>);
}

pub fn fread<I : Source, C: Consumer>(s: &mut PeekableSource<I>, c: C) {
    Consumer::consume(c, s)
}

pub struct Whitespace;

impl Consumer for Whitespace {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>) {
        while let Some(&c) = s.peek() {
            match c {
                b' ' | b'\t' ... b'\r' => { s.consume(); },
                _ => { break; },
            }
        }
    }
}

pub trait FromStrRadix : Sized {
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
}

pub struct Int<'a, T : 'a + FromStrRadix>(&'a mut T, u32);

pub fn bin<'a, T : 'a + FromStrRadix>(i: &'a mut T) -> Int<'a, T> {
    Int(i, 2)
}

pub fn oct<'a, T : 'a + FromStrRadix>(i: &'a mut T) -> Int<'a, T> {
    Int(i, 8)
}

pub fn hex<'a, T : 'a + FromStrRadix>(i: &'a mut T) -> Int<'a, T> {
    Int(i, 16)
}

fn is_digit(c: u8, radix: u32) -> bool {
    let d =
        match c {
            b'0' ... b'9' => { c - b'0' },
            b'A' ... b'Z' => { c - b'A' + 10u8 },
            b'a' ... b'z' => { c - b'a' + 10u8 },
            _ => { return false; },
        };
    (d as u32) < radix
}

impl<'a, T : 'a + FromStrRadix> Consumer for Int<'a, T> {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>) {
        let buf = &mut [0;40];
        let sink = &mut SliceSink::new(buf);

        match s.peek() {
            Some(&b'-') => {
                Sink::write(sink, b'-');
                s.consume();
            },
            _ => {
            },
        }

        while let Some(&c) = s.peek() {
            if !is_digit(c, self.1) {
                break;
            }

            Sink::write(sink, c);
            s.consume();
        }

        let s = unsafe {
            from_utf8_unchecked(buf.get_unchecked(..sink.offset()))
        };

        *self.0 = FromStrRadix::from_str_radix(s, self.1).unwrap()
    }
}

macro_rules! int {
    ($t:ty) => (
        impl FromStrRadix for $t {
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                <$t>::from_str_radix(src, radix)
            }
        }

        impl<'a> Consumer for &'a mut $t {
            fn consume<I : Source>(self, s: &mut PeekableSource<I>) {
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
    #[should_panic]
    fn test_unsigned_mismatch() {
        let source = &mut SliceSource::new(b"g");
        let mut x = 0usize;
        fread(source, hex(&mut x));
    }

    #[test]
    #[should_panic]
    fn test_unsigned_mismatch_empty() {
        let source = &mut SliceSource::new(b"");
        let mut x = 0usize;
        fread(source, hex(&mut x));
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
    #[should_panic]
    fn test_signed_mismatch_empty() {
        let source = &mut SliceSource::new(b"");
        let mut x = 0isize;
        fread(source, &mut x);
    }

    #[test]
    #[should_panic]
    fn test_signed_mismatch_sign() {
        let source = &mut SliceSource::new(b"-");
        let mut x = 0isize;
        fread(source, &mut x);
    }
}
