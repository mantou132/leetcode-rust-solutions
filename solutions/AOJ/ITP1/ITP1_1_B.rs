#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut x : isize = default();
    scanf!(stdin, "%d", &mut x);
    printf!(stdout, "%d\n", x*x*x);
}
