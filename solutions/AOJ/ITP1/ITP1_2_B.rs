#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);
    let (mut a, mut b, mut c): (isize, isize, isize) = default();
    io::scan(stdin, (&mut a, &mut b, &mut c))?;
    io::print(stdout,
              (if (a < b) && (b < c) {
                  "Yes"
               } else {
                  "No"
              },"\n"))?;
    Ok(())
}
