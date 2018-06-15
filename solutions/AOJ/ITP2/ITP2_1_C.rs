#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = unsafe { &mut STDOUT };
    let mut n : isize = default();
    read!(&mut n);

    let list = &mut DoublyLinkedList::new();
    let mut cursor = list.front();

    for _ in 0..n {
        let mut op : isize = default();
        read!(&mut op);
        if op == 0 {
            let mut x : isize = default();
            read!(&mut x);
            cursor = list.insert_before(x, cursor);
        } else if op == 1 {
            let mut d : isize = default();
            read!(&mut d);

            if d < 0 {
                for _ in 0..(-d) {
                    cursor = list.prev(cursor);
                }
            } else {
                for _ in 0..d {
                    cursor = list.next(cursor);
                }
            }
        } else if op == 2 {
            let next = list.next(cursor);
            list.remove(cursor);
            cursor = next;
        }
    }

    while !Deque::is_empty(list) {
        printf!(stdout, "%d\n", Deque::pop_front(list));
    }
}
