#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
use porus::prelude::*;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() -> isize {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut n : isize = default();

    scanf!(stdin, " %d", &mut n);

    if n <= 0 {
        return 0;
    }

    let a = array![0isize; n];

    for i in 0..n {
        scanf!(stdin, " %d", &mut a[i]);
    }

    printf!(stdout, "%d", a[n-1]);
    for i in 1..n {
        printf!(stdout, " %d", a[n-1-i]);
    }
    printf!(stdout, "\n");
    0
}
