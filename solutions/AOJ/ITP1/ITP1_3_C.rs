#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    loop {
        let x : isize = read!();
        let y : isize = read!();
        if (x == 0) && (y == 0) {
            break;
        }
        writelnf!("{:d} {:d}", Ord::min(x,y), Ord::max(x,y));
    }
}
