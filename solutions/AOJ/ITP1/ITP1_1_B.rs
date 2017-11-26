#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut x : isize = default();
    scanf!(stdin, "%d", &mut x);
    printf!(stdout, "%d\n", x*x*x);
}
