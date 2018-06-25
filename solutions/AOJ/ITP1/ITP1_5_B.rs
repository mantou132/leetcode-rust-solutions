#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    loop {
        let (mut h, mut w) : (isize, isize) = default();
        read!(&mut h, &mut w);
        if (h == 0) && (w == 0) {
            break;
        }

        for _ in 0..w {
            writef!("#");
        }
        writelnf!("");

        for _ in 0..(h-2) {
            writef!("#");
            for _ in 0..(w-2) {
                writef!(".");
            }
            writelnf!("#");
        }

        for _ in 0..w {
            writef!("#");
        }
        writelnf!("");

        writelnf!("");
    }
}
