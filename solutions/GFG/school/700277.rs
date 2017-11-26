// http://practice.geeksforgeeks.org/problems/print-the-pattern-set-1/1

#![feature(proc_macro)]
extern crate porus_macros;
#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());

    let mut t : isize = default();
    scanf!(stdin, " %d", &mut t);

    for _ in 0..t {
        let mut n : isize = default();
        scanf!(stdin, " %d", &mut n);

        for i in (1..n+1).rev() {
            for j in (1..n+1).rev() {
                for _ in 0..i {
                    printf!(stdout, "%d ", j);
                }
            }

            printf!(stdout, "$");
        }

        printf!(stdout, "\n");
    }
}
