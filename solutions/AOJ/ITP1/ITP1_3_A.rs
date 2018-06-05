#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = &mut io::stdout();
    for _ in 0..1000 {
        printf!(stdout, "Hello World\n");
    }
}
