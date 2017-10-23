use super::traits::*;


pub struct Peekable<T: Iterator> {
    iter: T,
    item: Option<T::Item>,
}


impl<T: Iterator> PeekingIterator for Peekable<T> {
    type Item = T::Item;

    fn peek<'a>(&'a mut self) -> Option<&'a Self::Item> {
        self.item.as_ref()
    }

    fn consume(&mut self) {
        self.item = self.iter.next()
    }
}


impl<T: Iterator<Item=u8>> InputStream for Peekable<T> {
}


pub fn peeking<T: Iterator>(mut iter: T) -> Peekable<T> {
    let item = iter.next();
    Peekable {
        iter: iter,
        item: item,
    }
}
