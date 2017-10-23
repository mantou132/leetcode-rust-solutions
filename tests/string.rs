#[macro_use]
extern crate porus;

use porus::traits::*;
use porus::io::{read_string, read_string_until, write};
use porus::ctype::isnewline;

pub mod common;
use common::io;


#[test]
fn test_eq() {
    let a = str!(b"abcd");
    let b = str!(b"abcd");
    let c = str!(b"abcde");
    assert!(a == b);
    assert!(a != c);
}


#[test]
fn test_iter() {
    let a = str!(b"abc");
    let mut i = a.iter();

    assert!(b'a' == *(i.next().unwrap()));
    assert!(b'b' == *(i.next().unwrap()));
    assert!(b'c' == *(i.next().unwrap()));
    assert!(i.next().is_none());
}


#[test]
fn test_add() {
    let a = str!(b"abcdefghijklmnopqrstuvwxyz");
    let b = str!(b"0");
    let c = str!(b"1");

    assert!(a + b == a + b);
    assert!(a + c == a + c);
    assert!(a != &(a + b));
    assert!(a != &(a + c));
    assert!(a + b != a + c);
}


#[test]
fn test_mul() {
    let a = str!(b"abc");
    let b = str!(b"abcabcabc");
    assert!(a == &(a * 1));
    assert!(b == &(a * 3));
}


#[test]
fn test_read_string() {
    let bytes = b"abc";
    let s = str!(bytes);
    let stream = &mut io::InputStream::new(bytes);
    let t = &read_string(stream, bytes.len());
    assert!(s == t);

    let bytes = b"abcdefghijklmnopqrstuvwxyz";
    let s = str!(bytes);
    let stream = &mut io::InputStream::new(bytes);
    let t = &read_string(stream, bytes.len());
    assert!(s == t);
}


#[test]
fn test_read_string_until() {
    let bytes = b"abcdefghijklmnopqrstuvwxyz\n";
    let stream = &mut io::InputStream::new(bytes);
    let s = &read_string_until(stream, isnewline, bytes.len());
    assert!(s == str!(b"abcdefghijklmnopqrstuvwxyz"));
}


#[test]
fn test_read_string_overflow() {
    let s = str!(b"abcdefghijklmnopqrstuvwxy");
    let stream = &mut io::InputStream::new(b"abcdefghijklmnopqrstuvwxyz");
    let t = &read_string(stream, 25);
    assert!(s == t);
}


#[test]
fn test_write_string() {
    let bytes = b"abcdefghijklmnopqrstuvwxyz";
    let array = &mut [0;26];
    {
        let stream = &mut io::OutputStream::new(array);
        write(stream, str!(bytes));
    }

    assert!(array == bytes);
}
