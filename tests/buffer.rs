#[macro_use]
extern crate porus;

use porus::traits::*;
use porus::collections::{Buffer, FixedCapacity};

pub mod common;
use common::drop::{Counter, Item};


#[test]
fn test_drop() {
    let counter = Counter::new();
    {
        let stack = &mut Buffer::<Item>::new_with_capacity(5);

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
        let deque = &mut Buffer::<Item>::new_with_capacity(5);
        Deque::push_back(deque, Item::new(counter.clone()));
        ListMut::set(deque, 0, Item::new(counter.clone()));
    }

    assert!(counter.borrow().count() == 2)
}


#[test]
fn test_list() {
    common::list::test_list(&mut Buffer::<usize>::new(vec![0;5].into_iter()));
}


#[test]
fn test_deque() {
    common::deque::test_deque(&mut Buffer::<usize>::new_with_capacity(0));
}


#[test]
#[should_panic(expected="empty")]
fn test_deque_empty_pop_front() {
    common::deque::test_empty_pop_front(&mut Buffer::<usize>::new_with_capacity(5));

}


#[test]
#[should_panic(expected="empty")]
fn test_deque_empty_pop_back() {
    common::deque::test_empty_pop_back(&mut Buffer::<usize>::new_with_capacity(5));
}


#[test]
#[should_panic(expected="overflow")]
fn test_deque_bounded_push_front_overflow() {
    common::deque::test_bounded_push_front_overflow(&mut Buffer::<usize, FixedCapacity>::new_with_capacity(5));
}


#[test]
#[should_panic(expected="overflow")]
fn test_deque_bounded_push_back_overflow() {
    common::deque::test_bounded_push_back_overflow(&mut Buffer::<usize, FixedCapacity>::new_with_capacity(5));
}


#[test]
fn test_deque_unbounded_grow() {
    common::deque::test_unbounded_grow(&mut Buffer::<usize>::new_with_capacity(0));
}


#[test]
fn test_deque_unbounded_reserve() {
    common::deque::test_unbounded_reserve(&mut Buffer::<usize>::new_with_capacity(0));
}


#[test]
fn test_stack() {
    common::stack::test_stack(&mut Buffer::<usize>::new_with_capacity(5));
}


#[test]
#[should_panic(expected="empty")]
fn test_stack_empty() {
    common::stack::test_empty(&mut Buffer::<usize>::new_with_capacity(5));
}


#[test]
#[should_panic(expected="overflow")]
fn test_bounded_stack_overflow() {
    common::stack::test_bounded_overflow(&mut Buffer::<usize, FixedCapacity>::new_with_capacity(5));
}


#[test]
fn test_unbounded_stack_grow() {
    common::stack::test_unbounded_grow(&mut Buffer::<usize>::new_with_capacity(0));
}


#[test]
fn test_unbounded_stack_shrink() {
    common::stack::test_unbounded_shrink(&mut Buffer::<usize>::new_with_capacity(0));
}


#[test]
fn test_unbounded_reserve() {
    common::unbounded::test_reserve(&mut Buffer::<usize>::new_with_capacity(0));
}


#[test]
fn test_unbounded_shrink() {
    let stack = &mut Buffer::<usize>::new_with_capacity(0);

    for i in 0..11 {
        Stack::push(stack, i);
    }

    common::unbounded::test_shrink(stack);

    for i in (0..11).rev() {
        assert!(Stack::pop(stack) == i);
    }
}
