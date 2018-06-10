use super::compat::prelude::*;

mod map;
use self::map::{Map, MapRef, MapRefMut};

pub trait Iter {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn map<T, Fn : FnMut(Self::Item) -> T>(self, f: Fn) -> Map<Self, T, Fn>
        where Self : Sized
    {
        Map::new(self, f)
    }

    fn foreach<Fn : FnMut(Self::Item) -> ()>(self, f: Fn)
        where Self : Sized
    {
        let mut iter = Iter::map(self, f);
        while let Some(()) = iter.next() {
        }
    }
}

pub trait IterRef {
    type Item;

    fn next(&mut self) -> Option<&Self::Item>;

    fn map<T, Fn : FnMut(&Self::Item) -> T>(self, f: Fn) -> MapRef<Self, T, Fn>
        where Self : Sized
    {
        MapRef::new(self, f)
    }

    fn foreach<Fn : FnMut(&Self::Item) -> ()>(self, f: Fn)
        where Self : Sized
    {
        let mut iter = IterRef::map(self, f);
        while let Some(()) = iter.next() {
        }
    }
}

pub trait IterRefMut {
    type Item;

    fn next(&mut self) -> Option<&mut Self::Item>;

    fn map<T, Fn : FnMut(&mut Self::Item) -> T>(self, f: Fn) -> MapRefMut<Self, T, Fn>
        where Self : Sized
    {
        MapRefMut::new(self, f)
    }

    fn foreach<Fn : FnMut(&mut Self::Item) -> ()>(self, f: Fn)
        where Self : Sized
    {
        let mut iter = IterRefMut::map(self, f);
        while let Some(()) = iter.next() {
        }
    }
}
