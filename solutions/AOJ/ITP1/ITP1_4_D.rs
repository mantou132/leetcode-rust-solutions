#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let mut n : isize = default();
    read!(&mut n);

    let mut min = isize::max_value();
    let mut max = isize::min_value();
    let mut sum : isize = 0;

    for _ in 0..n {
        let mut a : isize = default();
        read!(&mut a);

        min = Ord::min(a, min);
        max = Ord::max(a, max);
        sum = sum + a;
    }

    writelnf!("{min:d} {max:d} {sum:d}");
}
