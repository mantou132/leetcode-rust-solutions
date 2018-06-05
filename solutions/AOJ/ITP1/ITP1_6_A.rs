#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut n : isize = default();

    scanf!(stdin, " %d", &mut n);

    if n <= 0 {
        return;
    }

    let a = array![0isize; n];

    foreach! { e in list::iter_mut(a) =>
        scanf!(stdin, " %d", e);
    }

    let b = slice!(a, [,,-1]);

    printf!(stdout, "%d", b[0]);
    for i in 1..n {
        printf!(stdout, " %d", b[i]);
    }
    printf!(stdout, "\n");
}
