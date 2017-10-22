#![cfg_attr(not(debug_assertions), no_main)]

extern crate porus;
use porus::io::*;
use porus::ctype::isspace;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdin = &mut porus::io::file::input(0, 1024);
    let stdout = &mut porus::io::file::output(1, 1024);

    let t = read_u32(ignore(stdin, isspace));

    for _ in 0..t {
        let n = read_u32(ignore(stdin, isspace));

        for i in (1..n+1).rev() {
            for j in (1..n+1).rev() {
                for _ in 0..i {
                    write_u32(stdout, j);
                    write_s(stdout, b" ");
                }
            }

            write_s(stdout, b"$");
        }

        write_s(stdout, b"\n");
    }
}
