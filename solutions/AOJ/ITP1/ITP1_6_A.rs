#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let n : isize = read!();
    let a = &mut Array::<isize>::new_from_iter((0..n).map(|_| read!()));
    let r = slice!(a, [,,-1]);
    writelnf!("{}", join(f!(" "), list::iter(r).map(|e| f!("{e:d}"))));
}
