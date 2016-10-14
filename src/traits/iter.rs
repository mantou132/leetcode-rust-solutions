pub use core::iter::Iterator;

pub trait PeekingIterator {
    type Item;
    fn peek(&mut self) -> Option<&Self::Item>;
    fn consume(&mut self);
}
