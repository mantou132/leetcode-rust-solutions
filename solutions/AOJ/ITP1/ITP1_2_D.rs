#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
use porus::prelude::*;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let (mut w, mut h, mut x, mut y, mut r): (int, int, int, int ,int) = default();
    scanf!(stdin, " %d %d %d %d %d", &mut w, &mut h, &mut x, &mut y, &mut r);
    printf!(stdout,
            "%s\n",
            if (r <= x) && (x <= (w - r)) && (r <= y) && (y <= (h - r)) {
                "Yes"
            } else {
                "No"
            });
}
