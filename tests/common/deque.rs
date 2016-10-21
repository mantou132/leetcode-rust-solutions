use porus::traits::{Collection, Bounded, Unbounded, Deque};


pub fn test_deque<T: Deque<Item=usize>>(deque: &mut T) {
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


pub fn test_empty_pop_front<T: Deque<Item=usize>>(deque: &mut T) {
    Deque::push_back(deque, 1);
    Deque::push_back(deque, 2);
    Deque::pop_front(deque);
    Deque::pop_front(deque);
    assert!(Deque::is_empty(deque));
    Deque::pop_front(deque);
}


pub fn test_empty_pop_back<T: Deque<Item=usize>>(deque: &mut T) {
    Deque::push_front(deque, 1);
    Deque::push_front(deque, 2);
    Deque::pop_back(deque);
    Deque::pop_back(deque);
    assert!(Deque::is_empty(deque));
    Deque::pop_back(deque);
}


pub fn test_bounded_push_front_overflow<T: Bounded + Deque<Item=usize>>(deque: &mut T) {
    for i in 0..6 {
        Deque::push_front(deque, i);
    }
}


pub fn test_bounded_push_back_overflow<T: Bounded + Deque<Item=usize>>(deque: &mut T) {
    for i in 0..6 {
        Deque::push_back(deque, i);
    }
}


pub fn test_unbounded_grow<T: Unbounded + Deque<Item=usize>>(deque: &mut T) {

    for i in 0..10 {
        Deque::push_back(deque, i);
    }

    assert!(Unbounded::capacity(deque) == 10);
    Deque::push_back(deque, 10);
    assert!(Unbounded::capacity(deque) > 10);
}


pub fn test_unbounded_reserve<T: Unbounded + Deque<Item=usize>>(deque: &mut T) {

    for i in 0..5 {
        Deque::push_back(deque, i);
    }

    Unbounded::reserve(deque, 20);
    assert!(Unbounded::capacity(deque) == 20);

    for i in 0..5 {
        assert!(Deque::pop_front(deque) == i);
    }

    Unbounded::shrink_to_fit(deque);
    assert!(Unbounded::capacity(deque) < 20);

    for i in 0..5 {
        Deque::push_front(deque, i);
    }

    Unbounded::reserve(deque, 20);
    assert!(Unbounded::capacity(deque) == 20);

    for i in 0..5 {
        assert!(Deque::pop_back(deque) == i);
    }

    Unbounded::shrink_to_fit(deque);
    assert!(Unbounded::capacity(deque) < 20);
}
