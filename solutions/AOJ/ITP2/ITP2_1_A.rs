#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = unsafe { (&mut STDIN, &mut STDOUT) };
    let mut n : isize = default();

    scanf!(stdin, "%d", &mut n);

    let vec = array![0isize; 0];

    for _ in 0..n {
        let mut op : isize = default();
        scanf!(stdin, " %d", &mut op);
        if op == 0 {
            let mut x : isize = default();
            scanf!(stdin, " %d", &mut x);
            Stack::push(vec, x);
        } else if op == 1 {
            let mut p : isize = default();
            scanf!(stdin, " %d", &mut p);
            printf!(stdout, "%d\n", vec[p]);
        } else if op == 2 {
            Stack::pop(vec);
        }
    }
}
