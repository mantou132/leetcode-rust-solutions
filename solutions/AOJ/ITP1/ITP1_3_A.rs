#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    for _ in 0..1000 {
        write(f!("Hello World\n"));
    }
}
