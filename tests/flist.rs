#[macro_use]
extern crate porus;

use porus::traits::*;
use porus::collections::flist;

pub mod common;
use common::drop::{Counter, Item};


#[test]
fn test_drop() {
    let counter = Counter::new();
    {
        let stack = &mut flist::new();

        for _ in 0..5 {
            Stack::push(stack, Item::new(counter.clone()));
        }
    }

    assert!(counter.borrow().count() == 5)
}


#[test]
fn test_stack() {
    common::stack::test_stack(&mut flist::new());
    common::stack::test_stack(&mut flist::with_capacity(5));
}


#[test]
#[should_panic(expected="empty")]
fn test_stack_empty() {
    common::stack::test_empty(&mut flist::new());
}


#[test]
#[should_panic(expected="overflow")]
fn test_bounded_stack_overflow() {
    common::stack::test_bounded_overflow(&mut flist::with_capacity(5));
}
