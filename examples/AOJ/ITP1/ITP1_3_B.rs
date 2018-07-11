#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let mut i : isize = 1;
    loop {
        let x : isize = read!();
        if x == 0 {
            break;
        }
        writelnf!("Case {:d}: {:d}", i, x);
        i += 1;
    }
}
