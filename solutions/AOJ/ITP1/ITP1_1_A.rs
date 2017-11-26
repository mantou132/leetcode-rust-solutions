#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = &mut io::stdout();
    printf!(stdout, "Hello World\n");
}
