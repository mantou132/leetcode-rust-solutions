#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
use porus::prelude::*;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut t: int = default();
    scanf!(stdin, "%d", &mut t);
    let s = t % 60;
    let mut m = t / 60;
    let h = m / 60;
    m = m % 60;
    printf!(stdout, "%d:%d:%d\n", h, m, s);
}
