#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);
    let mut x : usize = default();
    io::scan(stdin, (&mut x,))?;
    io::print(stdout, (x*x*x, "\n"))?;
    Ok(())
}
