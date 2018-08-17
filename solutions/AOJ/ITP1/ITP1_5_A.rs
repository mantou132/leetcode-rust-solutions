#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    loop {
        let h: isize = read!();
        let w: isize = read!();
        if (h == 0) && (w == 0) {
            break;
        }

        for _ in 0..h {
            for _ in 0..w {
                writef!("#");
            }
            writelnf!("");
        }
        writelnf!("");
    }
}
