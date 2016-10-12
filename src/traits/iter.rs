use core::option::Option;

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait PeekingIterator {
    type Item;
    fn peek(&mut self) -> Option<&Self::Item>;
    fn consume(&mut self);
}
