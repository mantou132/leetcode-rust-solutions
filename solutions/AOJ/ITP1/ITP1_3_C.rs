#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = unsafe { &mut STDOUT };

    loop {
        let (mut x, mut y) : (isize, isize) = default();
        read!(&mut x, &mut y);
        if (x == 0) && (y == 0) {
            break;
        }
        printf!(stdout, "%d %d\n", Ord::min(x,y), Ord::max(x,y));
    }
}
