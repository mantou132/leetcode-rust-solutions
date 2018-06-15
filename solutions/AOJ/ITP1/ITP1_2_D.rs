#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = unsafe { &mut STDOUT };
    let (mut w, mut h, mut x, mut y, mut r): (isize, isize, isize, isize ,isize) = default();
    read!(&mut w, &mut h, &mut x, &mut y, &mut r);
    printf!(stdout,
            "%s\n",
            if (r <= x) && (x <= (w - r)) && (r <= y) && (y <= (h - r)) {
                "Yes"
            } else {
                "No"
            });
}
