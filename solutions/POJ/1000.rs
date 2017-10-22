#![cfg_attr(not(debug_assertions), no_main)]

extern crate porus;
use porus::io::*;
use porus::ctype::isspace;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdin = &mut file::input(0, 1024);
    let stdout = &mut file::output(1, 1024);
    let a: i32 = read(ignore(stdin, isspace));
    let b: i32 = read(ignore(stdin, isspace));
    write(stdout, a+b);
    write_char(stdout, b'\n');
}
