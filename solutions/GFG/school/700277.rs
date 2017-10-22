#![cfg_attr(not(debug_assertions), no_main)]

extern crate porus;
use porus::io::*;
use porus::ctype::isspace;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdin = &mut file::input(0, 1024);
    let stdout = &mut file::output(1, 1024);

    let t : u32 = read(ignore(stdin, isspace));

    for _ in 0..t {
        let n : u32 = read(ignore(stdin, isspace));

        for i in (1..n+1).rev() {
            for j in (1..n+1).rev() {
                for _ in 0..i {
                    write(stdout, j);
                    write_char(stdout, b' ');
                }
            }

            write_char(stdout, b'$');
        }

        write_char(stdout, b'\n');
    }
}
