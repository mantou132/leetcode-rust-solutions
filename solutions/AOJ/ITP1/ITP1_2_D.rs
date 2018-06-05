#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let (mut w, mut h, mut x, mut y, mut r): (isize, isize, isize, isize ,isize) = default();
    scanf!(stdin, " %d %d %d %d %d", &mut w, &mut h, &mut x, &mut y, &mut r);
    printf!(stdout,
            "%s\n",
            if (r <= x) && (x <= (w - r)) && (r <= y) && (y <= (h - r)) {
                "Yes"
            } else {
                "No"
            });
}
