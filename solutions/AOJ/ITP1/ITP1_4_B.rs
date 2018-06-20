#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let mut r : f64 = default();
    read!(&mut r);
    writelnf!("{:.6f} {:.6f}", PI * r * r, PI * 2.0 * r);
}
