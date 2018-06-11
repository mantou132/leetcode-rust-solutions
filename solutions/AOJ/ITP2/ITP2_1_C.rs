#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut n : isize = default();

    scanf!(stdin, "%d", &mut n);

    let list = &mut DoublyLinkedList::new();
    let mut cursor = list.front();

    for _ in 0..n {
        let mut op : isize = default();
        scanf!(stdin, " %d", &mut op);
        if op == 0 {
            let mut x : isize = default();
            scanf!(stdin, " %d", &mut x);
            cursor = list.insert_before(x, cursor);
        } else if op == 1 {
            let mut d : isize = default();
            scanf!(stdin, " %d", &mut d);

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
