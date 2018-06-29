#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (mut x1, mut y1, mut x2, mut y2) : (f64, f64, f64, f64) = default();
    read!(&mut x1, &mut y1, &mut x2, &mut y2);
    writelnf!("{:.5f}", sqrt((y2 - y1) * (y2 - y1) + (x2 - x1) * (x2 - x1)));
}
