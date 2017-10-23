// http://practice.geeksforgeeks.org/problems/print-the-pattern-set-1/1

#![cfg_attr(not(debug_assertions), no_main)]

extern crate porus;
use porus::io::*;
use porus::ctype::isspace;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdin = &mut stdin(1024, isspace);
    let stdout = &mut stdout(1024);


    let t : usize = read(stdin);

    for _ in 0..t {
        let n : usize = read(stdin);

        for i in (1..n+1).rev() {
            for j in (1..n+1).rev() {
                for _ in 0..i {
                    write(stdout, (j, " "));
                }
            }

            write(stdout, " ");
        }

        write(stdout, "\n");
    }
}
