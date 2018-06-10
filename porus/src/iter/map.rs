use super::super::compat::prelude::*;
use super::{Iter, IterRef, IterRefMut};

pub struct Map<I : Iter, T, F : FnMut(I::Item) -> T> {
    it: I,
    f: F,
}

impl<I : Iter, T, F : FnMut(I::Item) -> T> Map<I, T, F> {
    pub fn new(it: I, f: F) -> Self {
        Map {
            it: it,
            f: f,
        }
    }
}

impl<I : Iter, T, F : FnMut(I::Item) -> T> Iter for Map<I, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match Iter::next(&mut self.it) {
            None => None,
            Some(x) => Some((self.f)(x)),
        }
    }
}

pub struct MapRef<I : IterRef, T, F : FnMut(&I::Item) -> T> {
    it: I,
    f: F,
}

impl<I : IterRef, T, F : FnMut(&I::Item) -> T> MapRef<I, T, F> {
    pub fn new(it: I, f: F) -> Self {
        MapRef {
            it: it,
            f: f,
        }
    }
}

impl<I : IterRef, T, F : FnMut(&I::Item) -> T> Iter for MapRef<I, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match IterRef::next(&mut self.it) {
            None => None,
            Some(x) => Some((self.f)(x)),
        }
    }
}


pub struct MapRefMut<I : IterRefMut, T, F : FnMut(&mut I::Item) -> T> {
    it: I,
    f: F,
}

impl<I : IterRefMut, T, F : FnMut(&mut I::Item) -> T> MapRefMut<I, T, F> {
    pub fn new(it: I, f: F) -> Self {
        MapRefMut {
            it: it,
            f: f,
        }
    }
}

impl<I : IterRefMut, T, F : FnMut(&mut I::Item) -> T> Iter for MapRefMut<I, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match IterRefMut::next(&mut self.it) {
            None => None,
            Some(x) => Some((self.f)(x)),
        }
    }
}
