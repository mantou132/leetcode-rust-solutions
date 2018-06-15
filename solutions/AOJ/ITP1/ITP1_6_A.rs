#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = unsafe { &mut STDOUT };
    let mut n : isize = default();

    read!(&mut n);

    if n <= 0 {
        return;
    }

    let a = array![0isize; n];

    list::iter_ref_mut(a).foreach(|e| { read!(e); });

    let b = slice!(a, [,,-1]);

    printf!(stdout, "%d", b[0]);
    for i in 1..n {
        printf!(stdout, " %d", b[i]);
    }
    printf!(stdout, "\n");
}
