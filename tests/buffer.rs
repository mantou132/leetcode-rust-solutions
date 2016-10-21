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
        let deque = &mut Buffer::<Item>::with_capacity(5);

        for _ in 0..5 {
            Deque::push_back(deque, Item::new(counter.clone()));
        }
    }

    assert!(counter.borrow().count() == 5)
}


#[test]
fn test_set_drop() {
    let counter = Counter::new();
    {
        let deque = &mut Buffer::<Item>::with_capacity(5);
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
    let deque = &mut Buffer::<usize>::with_capacity(0);

    for i in 0..5 {
        Deque::push_front(deque, i);
    }

    assert!(Collection::size(deque) == 5);

    for i in (0..5).rev() {
        assert!(Deque::front(deque) == Some(&(i)));
        assert!(Deque::pop_front(deque) == i);
    }

    assert!(Collection::size(deque) == 0);

    for i in 0..5 {
        Deque::push_front(deque, i);
    }

    assert!(Collection::size(deque) == 5);

    for i in 0..5 {
        assert!(Deque::back(deque) == Some(&(i)));
        assert!(Deque::pop_back(deque) == i);
    }

    assert!(Collection::size(deque) == 0);

    for i in 0..5 {
        Deque::push_back(deque, i);
    }

    assert!(Collection::size(deque) == 5);

    for i in (0..5).rev() {
        assert!(Deque::back(deque) == Some(&(i)));
        assert!(Deque::pop_back(deque) == i);
    }

    assert!(Collection::size(deque) == 0);

    for i in 0..5 {
        Deque::push_back(deque, i);
    }

    assert!(Collection::size(deque) == 5);

    for i in 0..5 {
        assert!(Deque::front(deque) == Some(&(i)));
        assert!(Deque::pop_front(deque) == i);
    }

    assert!(Collection::size(deque) == 0);
}


#[test]
#[should_panic(expected="empty")]
fn test_deque_empty_pop_front() {
    let deque = &mut Buffer::<usize>::with_capacity(5);
    Deque::push_back(deque, 1);
    Deque::push_back(deque, 2);
    Deque::pop_front(deque);
    Deque::pop_front(deque);
    assert!(Deque::is_empty(deque));
    Deque::pop_front(deque);
}


#[test]
#[should_panic(expected="empty")]
fn test_deque_empty_pop_back() {
    let deque = &mut Buffer::<usize>::with_capacity(5);
    Deque::push_front(deque, 1);
    Deque::push_front(deque, 2);
    Deque::pop_back(deque);
    Deque::pop_back(deque);
    assert!(Deque::is_empty(deque));
    Deque::pop_back(deque);
}


#[test]
#[should_panic(expected="overflow")]
fn test_deque_push_front_overflow() {
    let deque = &mut Buffer::<usize, FixedCapacity>::with_capacity(5);
    for i in 0..6 {
        Deque::push_front(deque, i);
    }
}


#[test]
#[should_panic(expected="overflow")]
fn test_deque_push_back_overflow() {
    let deque = &mut Buffer::<usize, FixedCapacity>::with_capacity(5);
    for i in 0..6 {
        Deque::push_back(deque, i);
    }
}


#[test]
fn test_deque_grow() {
    let deque = &mut Buffer::<usize>::with_capacity(0);

    for i in 0..11 {
        Deque::push_back(deque, i);
    }

    let capacity = Unbounded::capacity(deque);

    for _ in 0..5 {
        Deque::pop_front(deque);
    }

    assert!(Unbounded::capacity(deque) < capacity);
}


#[test]
fn test_deque_reserve() {
    let deque = &mut Buffer::<usize>::with_capacity(0);

    for i in 0..5 {
        Deque::push_back(deque, i);
    }

    Unbounded::reserve(deque, 20);
    assert!(Unbounded::capacity(deque) == 20);

    for i in 0..5 {
        assert!(Deque::pop_front(deque) == i);
    }


    let deque = &mut Buffer::<usize>::with_capacity(0);

    for i in 0..5 {
        Deque::push_front(deque, i);
    }

    Unbounded::reserve(deque, 20);
    assert!(Unbounded::capacity(deque) == 20);

    for i in 0..5 {
        assert!(Deque::pop_back(deque) == i);
    }
}


#[test]
fn test_deque_shrink() {
    let deque = &mut Buffer::<usize>::with_capacity(0);

    for i in 0..11 {
        Deque::push_back(deque, i);
    }

    Unbounded::shrink_to_fit(deque);
    assert!(Unbounded::capacity(deque) == 11);

    for i in 0..11 {
        assert!(Deque::pop_front(deque) == i);
    }


    let deque = &mut Buffer::<usize>::with_capacity(0);

    for i in 0..11 {
        Deque::push_front(deque, i);
    }

    Unbounded::shrink_to_fit(deque);
    assert!(Unbounded::capacity(deque) == 11);

    for i in 0..11 {
        assert!(Deque::pop_back(deque) == i);
    }
}


#[test]
fn test_stack() {
    common::stack::test_stack(&mut Buffer::<usize>::with_capacity(5));
}


#[test]
#[should_panic(expected="empty")]
fn test_stack_empty() {
    common::stack::test_empty(&mut Buffer::<usize>::with_capacity(5));
}


#[test]
#[should_panic(expected="overflow")]
fn test_bounded_stack_overflow() {
    common::stack::test_bounded_overflow(&mut Buffer::<usize, FixedCapacity>::with_capacity(5));
}


#[test]
fn test_unbounded_stack_grow() {
    common::stack::test_unbounded_grow(&mut Buffer::<usize>::with_capacity(0));
}


#[test]
fn test_unbounded_stack_shrink() {
    common::stack::test_unbounded_shrink(&mut Buffer::<usize>::with_capacity(0));
}


#[test]
fn test_unbounded_reserve() {
    common::unbounded::test_reserve(&mut Buffer::<usize>::with_capacity(0));
}


#[test]
fn test_unbounded_shrink() {
    let stack = &mut Buffer::<usize>::with_capacity(0);

    for i in 0..11 {
        Stack::push(stack, i);
    }

    common::unbounded::test_shrink(stack);

    for i in (0..11).rev() {
        assert!(Stack::pop(stack) == i);
    }
}
