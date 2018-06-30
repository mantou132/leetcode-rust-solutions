#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let n : isize = read!();

    'outer:
    for i in (0..n).map(|x| x+1) {
        if (i % 3) != 0 {
            let mut x = i;
            while (x % 10) != 3 {
                if x == 0 {
                    continue 'outer;
                }
                x /= 10;
            }
        }

        writef!(" {:d}", i);
    }
    writelnf!("");
}
