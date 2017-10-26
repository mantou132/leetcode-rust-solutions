#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);
    let mut t: usize = default();
    io::scan(stdin, (&mut t,))?;
    let s = t % 60;
    let mut m = t / 60;
    let h = m / 60;
    m = m % 60;
    io::print(stdout, (h,":",m,":",s,"\n"))?;
    Ok(())
}
