#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);
    let mut x : int = default();
    scanf!(stdin, "%d", &mut x)?;
    printf!(stdout, "%d\n", x*x*x)?;
    Ok(())
}
