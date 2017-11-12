#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let stdout = &mut io::stdout()?;
    io::print(stdout, ("Hello World\n",));
    Ok(())
}
