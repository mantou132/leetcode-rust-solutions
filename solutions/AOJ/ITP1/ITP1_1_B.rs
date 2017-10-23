#![cfg_attr(not(debug_assertions), no_main)]

extern crate porus;
use porus::io::*;
use porus::ctype::isspace;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdin = &mut stdin(1024, isspace);
    let stdout = &mut stdout(1024);
    let x : usize = read(stdin);
    write(stdout, (x*x*x,"\n"));
}
