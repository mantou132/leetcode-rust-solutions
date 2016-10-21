use porus::traits::{Collection, Bounded, Unbounded, Stack};


pub fn test_stack<T : Stack<Item=usize>>(stack : &mut T) {
    for i in 0..5 {
        Stack::push(stack, i);
    }

    assert!(Collection::size(stack) == 5);

    for i in (0..5).rev() {
        assert!(Stack::top(stack) == Some(&(i)));
        assert!(Stack::pop(stack) == i);
    }

    assert!(Collection::size(stack) == 0);
    assert!(Stack::top(stack) == None);
}


pub fn test_empty<T : Stack<Item=usize>>(stack : &mut T) {
    assert!(Stack::is_empty(stack));
    Stack::push(stack, 1);
    Stack::push(stack, 2);
    Stack::pop(stack);
    Stack::pop(stack);
    assert!(Stack::is_empty(stack));
    Stack::pop(stack);
}


pub fn test_bounded_overflow<T : Bounded + Stack<Item=usize>>(stack : &mut T) {
    for i in 0..6 {
        Stack::push(stack, i);
    }
}


pub fn test_unbounded_grow<T : Unbounded + Stack<Item=usize>>(stack : &mut T) {
    for i in 0..10 {
        Stack::push(stack, i);
    }

    assert!(Unbounded::capacity(stack) == 10);
    Stack::push(stack, 10);
    assert!(Unbounded::capacity(stack) > 10);
}


pub fn test_unbounded_shrink<T : Unbounded + Stack<Item=usize>>(stack : &mut T) {
    for i in 0..11 {
        Stack::push(stack, i);
    }

    let capacity = Unbounded::capacity(stack);

    for _ in 0..5 {
        Stack::pop(stack);
    }

    assert!(Unbounded::capacity(stack) < capacity);
}
