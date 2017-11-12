#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);
    let mut i : int = 1;
    loop {
        let mut x : int = default();
        scanf!(stdin, " %d", &mut x)?;
        if x == 0 {
            break;
        }
        io::print(stdout, ("Case ", i, ": ", x, "\n"));
        i += 1;
    }
    Ok(())
}
