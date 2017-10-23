#[macro_use]
extern crate porus;

use porus::io::{eof, ignore, Read, write};
use porus::ctype::isspace;

pub mod common;
use common::io;


#[test]
fn test_eof() {
    let stream = &mut io::InputStream::new(b" ");
    assert!(!eof(stream));

    let stream = &mut io::InputStream::new(b"");
    assert!(eof(stream));
}


#[test]
fn test_ignore_space() {
    let stream = &mut io::InputStream::new(b"    ");
    ignore(stream, &isspace);
    assert!(eof(stream));
}


#[test]
fn test_read_unsigned() {
    let stream = &mut io::InputStream::new(b"123");
    let u : usize = Read::read(stream);
    assert!(u == 123);
}


#[test]
fn test_read_read_signed() {
    let stream = &mut io::InputStream::new(b"-123");
    let i : isize = Read::read(stream);
    assert!(i == -123);
}


#[test]
fn test_write_unsigned() {
    let array = &mut [0;1];
    {
        let stream = &mut io::OutputStream::new(array);
        write(stream, 0u8);
    }

    assert!(array == b"0");

    let array = &mut [0;3];
    {
        let stream = &mut io::OutputStream::new(array);
        write(stream, 123u8);
    }

    assert!(array == b"123");
}


#[test]
fn test_write_signed() {
    let array = &mut [0;1];
    {
        let stream = &mut io::OutputStream::new(array);
        write(stream, 0i8);
    }

    assert!(array == b"0");

    let array = &mut [0;3];
    {
        let stream = &mut io::OutputStream::new(array);
        write(stream, 123i8);
    }

    assert!(array == b"123");

    let array = &mut [0;4];
    {
        let stream = &mut io::OutputStream::new(array);
        write(stream, -123i8);
    }

    assert!(array == b"-123");
}
