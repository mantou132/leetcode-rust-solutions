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
