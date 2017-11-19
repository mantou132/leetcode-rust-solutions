#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
use porus::prelude::*;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut i : int = 1;
    loop {
        let mut x : int = default();
        scanf!(stdin, " %d", &mut x);
        if x == 0 {
            break;
        }
        printf!(stdout, "Case %d: %d\n", i, x);
        i += 1;
    }
}
