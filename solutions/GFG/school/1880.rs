// http://practice.geeksforgeeks.org/problems/c-hello-world/0

#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (_stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);
    io::print(stdout, ("Hello World\n",))?;
    Ok(())
}
