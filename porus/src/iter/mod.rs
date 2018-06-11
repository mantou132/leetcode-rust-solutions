mod map;
use self::map::{Map, MapRef, MapRefMut};

mod filter;
use self::filter::{Filter, FilterRef, FilterRefMut};


pub trait Iter {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn map<T, Fn : FnMut(Self::Item) -> T>(self, f: Fn) -> Map<Self, T, Fn>
        where Self : Sized
    {
        Map::new(self, f)
    }

    fn filter<Fn : FnMut(&Self::Item) -> bool>(self, f: Fn) -> Filter<Self, Fn>
        where Self : Sized
    {
        Filter::new(self, f)
    }

    fn foreach<Fn : FnMut(Self::Item) -> ()>(self, f: Fn)
        where Self : Sized
    {
        let mut iter = Iter::map(self, f);
        while let Some(()) = iter.next() {
        }
    }

    fn count(mut self) -> isize
        where Self : Sized
    {
        let mut count = 0;
        while let Some(_) = self.next() {
            count += 1;
        }
        count
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

    fn filter<Fn : FnMut(&Self::Item) -> bool>(self, f: Fn) -> FilterRef<Self, Fn>
        where Self : Sized
    {
        FilterRef::new(self, f)
    }

    fn foreach<Fn : FnMut(&Self::Item) -> ()>(self, f: Fn)
        where Self : Sized
    {
        let mut iter = IterRef::map(self, f);
        while let Some(()) = iter.next() {
        }
    }

    fn count(mut self) -> isize
        where Self : Sized
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

    fn map<T, Fn : FnMut(&mut Self::Item) -> T>(self, f: Fn) -> MapRefMut<Self, T, Fn>
        where Self : Sized
    {
        MapRefMut::new(self, f)
    }

    fn filter<Fn : FnMut(&Self::Item) -> bool>(self, f: Fn) -> FilterRefMut<Self, Fn>
        where Self : Sized
    {
        FilterRefMut::new(self, f)
    }

    fn foreach<Fn : FnMut(&mut Self::Item) -> ()>(self, f: Fn)
        where Self : Sized
    {
        let mut iter = IterRefMut::map(self, f);
        while let Some(()) = iter.next() {
        }
    }

    fn count(mut self) -> isize
        where Self : Sized
    {
        let mut count = 0;
        while let Some(_) = self.next() {
            count += 1;
        }
        count
    }
}

mod convert;
pub use self::convert::into_iter;
