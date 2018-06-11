#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());

    loop {
        let (mut x, mut y) : (isize, isize) = default();
        scanf!(stdin, " %d %d", &mut x, &mut y);
        if (x == 0) && (y == 0) {
            break;
        }
        printf!(stdout, "%d %d\n", Ord::min(x,y), Ord::max(x,y));
    }
}
