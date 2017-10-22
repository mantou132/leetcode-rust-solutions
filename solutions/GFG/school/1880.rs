#![cfg_attr(not(debug_assertions), no_main)]

extern crate porus;
use porus::io::*;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdout = &mut porus::io::file::output(1, 1024);
    write_s(stdout, b"Hello World\n");
}
