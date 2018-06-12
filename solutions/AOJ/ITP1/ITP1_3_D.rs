#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = unsafe { (&mut STDIN, &mut STDOUT) };

    let (mut a, mut b, mut c) : (isize, isize, isize) = default();
    scanf!(stdin, " %d %d %d", &mut a, &mut b, &mut c);

    printf!(stdout, "%d\n", into_iter(a..=b).filter(|x| {(&c) % x == 0}).count());
}
