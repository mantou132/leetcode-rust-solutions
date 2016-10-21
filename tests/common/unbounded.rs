use porus::traits::{Collection, Unbounded};


pub fn test_reserve<T : Unbounded>(unbounded : &mut T) {
    Unbounded::reserve(unbounded, 20);
    assert!(Unbounded::capacity(unbounded) == 20);
}


pub fn test_shrink<T : Collection + Unbounded>(unbounded : &mut T) {
    assert!(Unbounded::capacity(unbounded) > Collection::size(unbounded));
    Unbounded::shrink_to_fit(unbounded);
    assert!(Unbounded::capacity(unbounded) == Collection::size(unbounded));
}
