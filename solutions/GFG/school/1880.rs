#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate porus;
use porus::io::*;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdout = &mut file::output(1, 1024);
    write(stdout, str!(b"Hello World\n"));
}
