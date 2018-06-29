#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let mut n : isize = default();
    read!(&mut n);

    let a = array![0; n];
    list::iter_ref_mut(a).foreach(|e| { read!(e); });

    let count = list::bubble_sort(a);

    writelnf!("{}", join(f!(" "), list::iter(a).map(|e| f!("{e:d}"))));
    writelnf!("{count:d}");
}
