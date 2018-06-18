#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let mut t: isize = default();
    read!(&mut t);
    let s = t % 60;
    let mut m = t / 60;
    let h = m / 60;
    m = m % 60;
    write(f!("{h:d}:{m:d}:{s:d}\n"));
}
