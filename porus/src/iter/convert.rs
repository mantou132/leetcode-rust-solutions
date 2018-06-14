use core::ops::RangeInclusive;
use super::Iter;

pub trait IntoIter {
    type Iter : Iter;

    fn into(self) -> Self::Iter;
}

pub fn into_iter<T : IntoIter>(x: T) -> T::Iter {
    IntoIter::into(x)
}

pub struct RangeIter {
    current: isize,
    end: isize,
}

impl Iter for RangeIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let current = self.current;
            self.current += 1;
            Some(current)
        }
    }
}


impl IntoIter for RangeInclusive<isize> {
    type Iter = RangeIter;

    fn into(self) -> Self::Iter {
        RangeIter {
            current: *self.start(),
            end: self.end() + 1,
        }
    }
}


pub struct SliceIter<'a, T : 'a + Copy> {
    s: &'a [T],
}

impl<'a, T : 'a + Copy> Iter for SliceIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.s.split_first() {
            Some((i,s)) => {
                self.s = s;
                Some(*i)
            },
            None => {
                None
            },
        }
    }
}

impl<'a, T : 'a + Copy> IntoIter for &'a[T] {
    type Iter = SliceIter<'a,T>;

    fn into(self) -> Self::Iter {
        SliceIter {
            s: self,
        }
    }
}
