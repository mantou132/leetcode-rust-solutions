#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let n : isize = read!();

    let a = array![0; n];
    list::iter_ref_mut(a).foreach(|e| { read!(e); });

    writelnf!("{}", join(f!(" "), list::iter(a).map(|e| f!("{e:d}"))));

    for i in 2..n+1 {
        list::sort::bubble_sorted(slice_mut!(a, [0, i]), &PartialOrd::lt);
        writelnf!("{}", join(f!(" "), list::iter(a).map(|e| f!("{e:d}"))));
    }
}
