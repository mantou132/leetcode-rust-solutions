#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let mut x : isize = default();
    read!(&mut x);
    let cube = x*x*x;
    write(f!("{cube:d}\n"));
}
