#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut x : isize = default();
    scanf!(stdin, "%d", &mut x);
    printf!(stdout, "%d\n", x*x*x);
}
