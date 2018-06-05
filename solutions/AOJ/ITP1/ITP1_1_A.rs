#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = &mut io::stdout();
    printf!(stdout, "Hello World\n");
}
