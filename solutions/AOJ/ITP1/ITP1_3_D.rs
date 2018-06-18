#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (mut a, mut b, mut c) : (isize, isize, isize) = default();
    read!(&mut a, &mut b, &mut c);
    io::writeln(f!("{:d}", into_iter(a..=b).filter(|x| {(&c) % x == 0}).count()));
}
