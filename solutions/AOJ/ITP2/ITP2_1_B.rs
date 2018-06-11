#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut n : isize = default();

    scanf!(stdin, "%d", &mut n);

    let buf = buffer![];

    for _ in 0..n {
        let mut op : isize = default();
        scanf!(stdin, " %d", &mut op);
        if op == 0 {
            let (mut d, mut x) : (isize, isize) = default();
            scanf!(stdin, " %d %d", &mut d, &mut x);
            if d == 0 {
                Deque::push_front(buf, x);
            } else if d == 1 {
                Deque::push_back(buf, x);
            }
        } else if op == 1 {
            let mut p : isize = default();
            scanf!(stdin, " %d", &mut p);
            printf!(stdout, "%d\n", buf[p]);
        } else if op == 2 {
            let mut d : isize = default();
            scanf!(stdin, " %d", &mut d);
            if d == 0 {
                Deque::pop_front(buf);
            } else if d == 1 {
                Deque::pop_back(buf);
            }
        }
    }
}
