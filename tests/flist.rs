#[macro_use]
extern crate porus;

use porus::traits::*;
use porus::collections::ForwardList;
use porus::storage::Pool;

pub mod common;
use common::drop::{Counter, Item};


#[test]
fn test_drop() {
    let counter = Counter::new();
    {
        let stack = &mut ForwardList::new();

        for _ in 0..5 {
            Stack::push(stack, Item::new(counter.clone()));
        }
    }

    assert!(counter.borrow().count() == 5)
}


#[test]
fn test_stack() {
    common::stack::test_stack(&mut ForwardList::new());
    common::stack::test_stack(&mut ForwardList::with_allocator(Pool::with_capacity(5)));
}


#[test]
#[should_panic(expected="empty")]
fn test_stack_empty() {
    common::stack::test_empty(&mut ForwardList::new());
}


#[test]
#[should_panic(expected="overflow")]
fn test_bounded_stack_overflow() {
    common::stack::test_bounded_overflow(&mut ForwardList::with_allocator(Pool::with_capacity(5)));
}
