#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);
    let (mut a, mut b, mut c): (int, int, int) = default();
    scanf!(stdin, " %d %d %d", &mut a, &mut b, &mut c)?;
    printf!(stdout,
            "%s\n",
            if (a < b) && (b < c) {
                "Yes"
            } else {
                "No"
            })?;
    Ok(())
}