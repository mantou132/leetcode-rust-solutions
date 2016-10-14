#[macro_use]
extern crate porus;

use porus::traits::*;
use porus::tests;
use porus::string::string;
use porus::io::{read_string, read_string_until, write_string};


#[test]
fn test_eq() {
    let a = &string(b"abcd");
    let b = &string(b"abcd");
    let c = &string(b"abcde");
    assert!(a == b);
    assert!(a != c);
}


#[test]
fn test_iter() {
    let a = &string(b"abc");
    let mut i = a.iter();

    assert!(b'a' == *(i.next().unwrap()));
    assert!(b'b' == *(i.next().unwrap()));
    assert!(b'c' == *(i.next().unwrap()));
    assert!(i.next().is_none());
}


#[test]
fn test_add() {
    let a = &string(b"abcdefghijklmnopqrstuvwxyz");
    let b = &string(b"0");
    let c = &string(b"1");

    assert!(a + b == a + b);
    assert!(a + c == a + c);
    assert!(a != &(a + b));
    assert!(a != &(a + c));
    assert!(a + b != a + c);
}


#[test]
fn test_mul() {
    let a = &string(b"abc");
    let b = &string(b"abcabcabc");
    assert!(a == &(a * 1));
    assert!(b == &(a * 3));
}


#[test]
fn test_read_string() {
    let bytes = b"abc";
    let s = &string(bytes);
    let stream = &mut tests::InputStream::new(bytes);
    let t = &read_string(stream, bytes.len());
    assert!(s == t);

    let bytes = b"abcdefghijklmnopqrstuvwxyz";
    let s = &string(bytes);
    let stream = &mut tests::InputStream::new(bytes);
    let t = &read_string(stream, bytes.len());
    assert!(s == t);
}


#[test]
fn test_read_string_until() {
    let bytes = b"abcdefghijklmnopqrstuvwxyz";
    let stream = &mut tests::InputStream::new(bytes);
    let s = &read_string_until(stream, b'z', bytes.len());
    assert!(s == &string(b"abcdefghijklmnopqrstuvwxy"));
}


#[test]
#[should_panic(expected="buffer overflow")]
fn test_read_string_overflow() {
    let stream = &mut tests::InputStream::new(b"abcdefghijklmnopqrstuvwxyz");
    read_string(stream, 25);
}


#[test]
fn test_write_string() {
    let bytes = b"abcdefghijklmnopqrstuvwxyz";
    let array = &mut [0;26];
    {
        let stream = &mut tests::OutputStream::new(array);
        write_string(stream, &string(bytes));
    }

    assert!(array == bytes);
}
