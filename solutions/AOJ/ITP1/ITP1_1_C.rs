#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (mut a, mut b): (isize, isize) = default();
    read!(&mut a, &mut b);
    let area = a*b;
    let perimeter = (a+b) * 2;
    write(f!("{area:d} {perimeter:d}\n"));
}
