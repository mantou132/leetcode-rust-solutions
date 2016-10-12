#[macro_use]
extern crate porus;

use porus::traits::{PeekingIterator, OutputStream};
use porus::io::*;

struct StringStream<'a> {
    s: &'a [u8],
}

impl<'a> PeekingIterator for StringStream<'a> {
    type Item = u8;

    fn peek<'b>(&'b mut self) -> Option<&'b Self::Item> {
        self.s.first()
    }

    fn consume(&mut self) {
        match self.s.split_first() {
            Some((_,s)) => self.s = s,
            _ => (),
        }
    }
}

struct ArrayStream<'a> {
    offset: usize,
    s: &'a mut [u8],
}

impl<'a> OutputStream for ArrayStream<'a> {
    fn write(&mut self, c: u8) {
        if self.offset == self.s.len() {
            abort!("buffer overflow");
        }

        self.s[self.offset] = c;
        self.offset += 1;
    }
}

impl<'a> ArrayStream<'a> {
    pub fn new(s: &'a mut[u8]) -> ArrayStream<'a> {
        ArrayStream {
            offset: 0,
            s: s,
        }
    }
}

#[test]
fn test_eof() {
    let stream = &mut StringStream {s: b" "};
    assert!(!eof(stream));

    let stream = &mut StringStream {s: b""};
    assert!(eof(stream));
}

#[test]
fn test_ignore_space() {
    let stream = &mut StringStream {s: b" "};
    assert!(eof(ignore_space(stream)));
}

#[test]
fn test_read_unsigned() {
    let stream = &mut StringStream {s: b"123"};
    let u = read_u8(stream);
    assert!(u == 123);
}

#[test]
fn test_read_read_signed() {
    let stream = &mut StringStream {s: b"-123"};
    let i = read_i8(stream);
    assert!(i == -123);
}

#[test]
fn test_write_unsigned() {
    let array = &mut [0;1];
    {
        let stream = &mut ArrayStream::new(array);
        write_u8(stream, 0u8);
    }

    assert!(array == b"0");

    let array = &mut [0;3];
    {
        let stream = &mut ArrayStream::new(array);
        write_u8(stream, 123u8);
    }

    assert!(array == b"123");
}

#[test]
fn test_write_signed() {
    let array = &mut [0;1];
    {
        let stream = &mut ArrayStream::new(array);
        write_i8(stream, 0i8);
    }

    assert!(array == b"0");

    let array = &mut [0;3];
    {
        let stream = &mut ArrayStream::new(array);
        write_i8(stream, 123i8);
    }

    assert!(array == b"123");

    let array = &mut [0;4];
    {
        let stream = &mut ArrayStream::new(array);
        write_i8(stream, -123i8);
    }

    println!("{} {} {} {}", array[0], array[1], array[2], array[3]);

    assert!(array == b"-123");
}
