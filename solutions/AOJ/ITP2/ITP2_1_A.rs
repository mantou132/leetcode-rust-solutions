#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let mut n : isize = default();
    read!(&mut n);

    let vec = array![0isize; 0];

    for _ in 0..n {
        let mut op : isize = default();
        read!(&mut op);
        if op == 0 {
            let mut x : isize = default();
            read!(&mut x);
            Stack::push(vec, x);
        } else if op == 1 {
            let mut p : isize = default();
            read!(&mut p);
            let x = vec[p];
            io::writeln(f!("{x:d}"));
        } else if op == 2 {
            Stack::pop(vec);
        }
    }
}
