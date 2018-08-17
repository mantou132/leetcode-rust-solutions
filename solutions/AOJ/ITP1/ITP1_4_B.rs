#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let r: f64 = read!();
    writelnf!("{:.6f} {:.6f}", PI * r * r, PI * 2.0 * r);
}
