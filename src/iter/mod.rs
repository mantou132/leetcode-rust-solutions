mod map;
use self::map::{MapRef, MapRefMut};

mod filter;
use self::filter::{FilterRef, FilterRefMut};

pub use core::iter::{IntoIterator, Iterator, Peekable};

pub fn into_iter<T: IntoIterator>(x: T) -> T::IntoIter {
    IntoIterator::into_iter(x)
}

pub trait IterRef {
    type Item;

    fn next(&mut self) -> Option<&Self::Item>;

    fn map<T, Fn: FnMut(&Self::Item) -> T>(self, f: Fn) -> MapRef<Self, T, Fn>
    where
        Self: Sized,
    {
        MapRef::new(self, f)
    }

    fn filter<Fn: FnMut(&Self::Item) -> bool>(self, f: Fn) -> FilterRef<Self, Fn>
    where
        Self: Sized,
    {
        FilterRef::new(self, f)
    }

    fn foreach<Fn: FnMut(&Self::Item) -> ()>(self, f: Fn)
    where
        Self: Sized,
    {
        for () in IterRef::map(self, f) {}
    }

    fn count(mut self) -> isize
    where
        Self: Sized,
    {
        let mut count = 0;
        while let Some(_) = self.next() {
            count += 1;
        }
        count
    }
}

pub trait IterRefMut {
    type Item;

    fn next(&mut self) -> Option<&mut Self::Item>;

    fn map<T, Fn: FnMut(&mut Self::Item) -> T>(self, f: Fn) -> MapRefMut<Self, T, Fn>
    where
        Self: Sized,
    {
        MapRefMut::new(self, f)
    }

    fn filter<Fn: FnMut(&Self::Item) -> bool>(self, f: Fn) -> FilterRefMut<Self, Fn>
    where
        Self: Sized,
    {
        FilterRefMut::new(self, f)
    }

    fn foreach<Fn: FnMut(&mut Self::Item) -> ()>(self, f: Fn)
    where
        Self: Sized,
    {
        for () in IterRefMut::map(self, f) {}
    }

    fn count(mut self) -> isize
    where
        Self: Sized,
    {
        let mut count = 0;
        while let Some(_) = self.next() {
            count += 1;
        }
        count
    }
}
