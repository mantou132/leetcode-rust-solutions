#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = &mut io::stdout();
    for _ in 0..1000 {
        printf!(stdout, "Hello World\n");
    }
}
