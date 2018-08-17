#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

use porus::io::read::Consumer;
use porus::io::{PeekableSource, Source};

enum Symbol {
    Operator(u8),
    Operand(isize),
}

use Symbol::Operand;
use Symbol::Operator;

impl Default for Symbol {
    fn default() -> Self {
        Operator(0)
    }
}

impl<'a> Consumer for &'a mut Symbol {
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        match s.peek() {
            None => false,
            Some(&b'+') => {
                *self = Operator(b'+');
                s.consume();
                true
            }
            Some(&b'-') => {
                *self = Operator(b'-');
                s.consume();
                true
            }
            Some(&b'*') => {
                *self = Operator(b'*');
                s.consume();
                true
            }
            Some(_) => {
                let mut x: isize = 0;
                if Consumer::consume(&mut x, s) {
                    *self = Operand(x);
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn solve() {
    let a = array![0isize; 0];

    while let Some(s) = read_opt!() {
        match s {
            Operand(x) => {
                stack::push(a, x);
            }
            Operator(b'+') => {
                let y = stack::pop(a);
                let x = stack::pop(a);
                stack::push(a, x + y);
            }
            Operator(b'-') => {
                let y = stack::pop(a);
                let x = stack::pop(a);
                stack::push(a, x - y);
            }
            Operator(b'*') => {
                let y = stack::pop(a);
                let x = stack::pop(a);
                stack::push(a, x * y);
            }
            Operator(_) => unreachable!(),
        }
    }

    writelnf!("{:d}", a[0]);
}
