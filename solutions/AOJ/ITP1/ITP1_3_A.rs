#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
use porus::prelude::*;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdout = &mut io::stdout();
    for _ in 0..1000 {
        printf!(stdout, "Hello World\n");
    }
}
