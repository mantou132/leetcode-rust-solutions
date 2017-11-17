// http://practice.geeksforgeeks.org/problems/c-hello-world/0

#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let stdout = &mut io::stdout()?;
    printf!(stdout, "Hello World\n")?;
    Ok(())
}
