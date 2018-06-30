#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let n : isize = read!();

    let buf = buffer![];

    for _ in 0..n {
        let op : isize = read!();
        if op == 0 {
            let d : isize = read!();
            let x : isize = read!();
            if d == 0 {
                Deque::push_front(buf, x);
            } else if d == 1 {
                Deque::push_back(buf, x);
            }
        } else if op == 1 {
            let p : isize = read!();
            writelnf!("{:d}", buf[p]);
        } else if op == 2 {
            let d : isize = read!();
            if d == 0 {
                Deque::pop_front(buf);
            } else if d == 1 {
                Deque::pop_back(buf);
            }
        }
    }
}
