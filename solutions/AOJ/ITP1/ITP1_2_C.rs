#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let (mut a, mut b, mut c): (isize, isize, isize) = default();
    scanf!(stdin, " %d %d %d", &mut a, &mut b, &mut c);
    printf!(stdout,
            "%d %d %d\n",
            Ord::min(Ord::min(a,b),c),
            Ord::max(Ord::max(Ord::min(a,b), Ord::min(b,c)), Ord::min(a,c)),
            Ord::max(Ord::max(a,b),c));
}
