use porus::traits::{Collection, List, ListMut};

pub fn test_list<T : ListMut<Item=usize>>(list : &mut T) {
    assert!(Collection::size(list) == 5);

    for i in 0..5 {
        assert!(List::get(list, i) == &0);
    }

    for i in 0..5 {
        ListMut::set(list, i, i);
    }

    for i in 0..5 {
        assert!(List::get(list, i) == &i);
    }
}
