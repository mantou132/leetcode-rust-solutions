use super::{IterRef, IterRefMut};

pub struct FilterRef<I: IterRef, F: FnMut(&I::Item) -> bool> {
    it: I,
    f: F,
}

impl<I: IterRef, F: FnMut(&I::Item) -> bool> FilterRef<I, F> {
    pub fn new(it: I, f: F) -> Self {
        FilterRef { it, f }
    }
}

impl<I: IterRef, F: FnMut(&I::Item) -> bool> IterRef for FilterRef<I, F> {
    type Item = I::Item;

    fn next(&mut self) -> Option<&Self::Item> {
        while let Some(x) = IterRef::next(&mut self.it) {
            if (self.f)(x) {
                return Some(x);
            }
        }
        None
    }
}

pub struct FilterRefMut<I: IterRefMut, F: FnMut(&I::Item) -> bool> {
    it: I,
    f: F,
}

impl<I: IterRefMut, F: FnMut(&I::Item) -> bool> FilterRefMut<I, F> {
    pub fn new(it: I, f: F) -> Self {
        FilterRefMut { it, f }
    }
}

impl<I: IterRefMut, F: FnMut(&I::Item) -> bool> IterRef for FilterRefMut<I, F> {
    type Item = I::Item;

    fn next(&mut self) -> Option<&Self::Item> {
        while let Some(x) = IterRefMut::next(&mut self.it) {
            if (self.f)(x) {
                return Some(x);
            }
        }
        None
    }
}
