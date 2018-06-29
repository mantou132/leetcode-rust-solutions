#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (mut a, mut b) : (isize, isize) = default();
    read!(&mut a, &mut b);
    let d = a / b;
    let r = a % b;
    let f = (a as f64) / (b as f64);

    writelnf!("{d:d} {r:d} {f:.6f}");
}