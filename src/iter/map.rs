use super::{IterRef, IterRefMut, Iterator};

pub struct MapRef<I: IterRef, T, F: FnMut(&I::Item) -> T> {
    it: I,
    f: F,
}

impl<I: IterRef, T, F: FnMut(&I::Item) -> T> MapRef<I, T, F> {
    pub fn new(it: I, f: F) -> Self {
        MapRef { it, f }
    }
}

impl<I: IterRef, T, F: FnMut(&I::Item) -> T> Iterator for MapRef<I, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match IterRef::next(&mut self.it) {
            None => None,
            Some(x) => Some((self.f)(x)),
        }
    }
}

pub struct MapRefMut<I: IterRefMut, T, F: FnMut(&mut I::Item) -> T> {
    it: I,
    f: F,
}

impl<I: IterRefMut, T, F: FnMut(&mut I::Item) -> T> MapRefMut<I, T, F> {
    pub fn new(it: I, f: F) -> Self {
        MapRefMut { it, f }
    }
}

impl<I: IterRefMut, T, F: FnMut(&mut I::Item) -> T> Iterator for MapRefMut<I, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match IterRefMut::next(&mut self.it) {
            None => None,
            Some(x) => Some((self.f)(x)),
        }
    }
}
