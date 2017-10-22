#![cfg_attr(not(debug_assertions), no_main)]

extern crate porus;
use porus::io::*;
use porus::ctype::{isupper, isnewline};

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdin = &mut file::input(0, 1024);
    let stdout = &mut file::output(1, 1024);

    let s = &read_string_until(stdin, isnewline, 100000);

    let mut count : usize = 1;
    for c in s.iter() {
        if isupper(*c) {
            count += 1;
        }
    }

    write(stdout, count);
    write_char(stdout, b'\n');
}
