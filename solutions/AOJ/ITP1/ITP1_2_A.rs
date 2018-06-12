#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = unsafe { (&mut STDIN, &mut STDOUT) };
    let (mut a, mut b): (isize, isize) = default();
    scanf!(stdin, " %d %d", &mut a, &mut b);
    printf!(stdout,
            "a %s b\n",
            match Ord::cmp(&a, &b) {
                Less => "<",
                Equal => "==",
                Greater => ">",
            });
}
