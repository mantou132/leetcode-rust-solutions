#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate porus;
use porus::io::*;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdin = &mut porus::io::file::input(0, 1024);
    let stdout = &mut porus::io::file::output(1, 1024);
    let a = read_i32(ignore_space(stdin));
    let b = read_i32(ignore_space(stdin));
    write_i32(stdout, a+b);
    write_s(stdout, b"\n");
}
