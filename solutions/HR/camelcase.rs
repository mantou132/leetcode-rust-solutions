#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate porus;
use porus::io::*;
use porus::ctype::{isupper, isnewline};

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdin = &mut porus::io::file::input(0, 1024);
    let stdout = &mut porus::io::file::output(1, 1024);

    let s = &read_string_until(stdin, isnewline, 100000);

    let mut count = 1usize;
    for c in s.iter() {
        if isupper(*c) {
            count += 1;
        }
    }

    write_usize(stdout, count);
    write_s(stdout, b"\n");
}
