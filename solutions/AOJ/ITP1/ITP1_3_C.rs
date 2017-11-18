#![cfg_attr(not(debug_assertions), no_main)]
#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);

    loop {
        let (mut x, mut y) : (int, int) = default();
        scanf!(stdin, " %d %d", &mut x, &mut y)?;
        if (x == 0) && (y == 0) {
            break;
        }
        printf!(stdout, "%d %d\n", Ord::min(x,y), Ord::max(x,y))?;
    }
    Ok(())
}
