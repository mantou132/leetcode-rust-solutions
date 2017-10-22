#[macro_use]
extern crate porus;

use porus::traits::*;
use porus::collections::{Array, FixedCapacity};

pub mod common;
use common::drop::{Counter, Item};


#[test]
fn test_drop() {
    let counter = Counter::new();
    {
        let stack = &mut Array::<Item>::new_with_capacity(5);

        for _ in 0..5 {
            Stack::push(stack, Item::new(counter.clone()));
        }
    }

    assert!(counter.borrow().count() == 5)
}


#[test]
fn test_set_drop() {
    let counter = Counter::new();
    {
        let stack = &mut Array::<Item>::new_with_capacity(5);
        Stack::push(stack, Item::new(counter.clone()));
        ListMut::set(stack, 0, Item::new(counter.clone()));
    }

    assert!(counter.borrow().count() == 2)
}


#[test]
fn test_list() {
    common::list::test_list(&mut Array::<usize>::new(vec![0;5].into_iter()));
}


#[test]
fn test_stack() {
    common::stack::test_stack(&mut Array::<usize>::new_with_capacity(5));
}


#[test]
#[should_panic(expected="empty")]
fn test_stack_empty() {
    common::stack::test_empty(&mut Array::<usize>::new_with_capacity(5));
}


#[test]
#[should_panic(expected="overflow")]
fn test_bounded_stack_overflow() {
    common::stack::test_bounded_overflow(&mut Array::<usize, FixedCapacity>::new_with_capacity(5));
}


#[test]
fn test_unbounded_stack_grow() {
    common::stack::test_unbounded_grow(&mut Array::<usize>::new_with_capacity(0));
}


#[test]
fn test_unbounded_stack_shrink() {
    common::stack::test_unbounded_shrink(&mut Array::<usize>::new_with_capacity(0));
}


#[test]
fn test_unbounded_reserve() {
    common::unbounded::test_reserve(&mut Array::<usize>::new_with_capacity(0));
}


#[test]
fn test_unbounded_shrink() {
    let stack = &mut Array::<usize>::new_with_capacity(0);

    for i in 0..11 {
        Stack::push(stack, i);
    }

    common::unbounded::test_shrink(stack);

    for i in (0..11).rev() {
        assert!(Stack::pop(stack) == i);
    }
}
