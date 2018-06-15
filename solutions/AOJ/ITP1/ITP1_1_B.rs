#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = unsafe { &mut STDOUT };
    let mut x : isize = default();
    read!(&mut x);
    printf!(stdout, "%d\n", x*x*x);
}
