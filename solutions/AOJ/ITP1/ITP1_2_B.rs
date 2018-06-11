#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let (mut a, mut b, mut c): (isize, isize, isize) = default();
    scanf!(stdin, " %d %d %d", &mut a, &mut b, &mut c);
    printf!(stdout,
            "%s\n",
            if (a < b) && (b < c) {
                "Yes"
            } else {
                "No"
            });
}
