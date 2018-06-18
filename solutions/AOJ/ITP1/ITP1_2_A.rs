#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (mut a, mut b): (isize, isize) = default();
    read!(&mut a, &mut b);
    io::writeln(
        f!("a {} b",
           match Ord::cmp(&a, &b) {
               Less => "<",
               Equal => "==",
               Greater => ">",
           }));
}
