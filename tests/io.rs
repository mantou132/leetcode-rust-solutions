#[macro_use]
extern crate porus;

use porus::tests;
use porus::io::*;
use porus::ctype::isspace;


#[test]
fn test_eof() {
    let stream = &mut tests::InputStream::new(b" ");
    assert!(!eof(stream));

    let stream = &mut tests::InputStream::new(b"");
    assert!(eof(stream));
}


#[test]
fn test_ignore_space() {
    let stream = &mut tests::InputStream::new(b"    ");
    assert!(eof(ignore(stream, isspace)));
}


#[test]
fn test_read_unsigned() {
    let stream = &mut tests::InputStream::new(b"123");
    let u = read_u8(stream);
    assert!(u == 123);
}


#[test]
fn test_read_read_signed() {
    let stream = &mut tests::InputStream::new(b"-123");
    let i = read_i8(stream);
    assert!(i == -123);
}


#[test]
fn test_write_unsigned() {
    let array = &mut [0;1];
    {
        let stream = &mut tests::OutputStream::new(array);
        write_u8(stream, 0u8);
    }

    assert!(array == b"0");

    let array = &mut [0;3];
    {
        let stream = &mut tests::OutputStream::new(array);
        write_u8(stream, 123u8);
    }

    assert!(array == b"123");
}


#[test]
fn test_write_signed() {
    let array = &mut [0;1];
    {
        let stream = &mut tests::OutputStream::new(array);
        write_i8(stream, 0i8);
    }

    assert!(array == b"0");

    let array = &mut [0;3];
    {
        let stream = &mut tests::OutputStream::new(array);
        write_i8(stream, 123i8);
    }

    assert!(array == b"123");

    let array = &mut [0;4];
    {
        let stream = &mut tests::OutputStream::new(array);
        write_i8(stream, -123i8);
    }

    assert!(array == b"-123");
}
