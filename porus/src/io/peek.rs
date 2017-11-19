use super::{Source, PeekableSource};

pub struct Peekable<S: Source> {
    source: S,
    item: Option<S::Item>,
}

impl<S: Source> Peekable<S> {
    pub fn new(mut s: S) -> Self {
        let item = Source::read(&mut s);
        Peekable {
            source: s,
            item: item,
        }
    }
}

impl<S: Source> PeekableSource for Peekable<S> {
    type Item = S::Item;

    fn peek<'a>(&'a mut self) -> Option<&'a Self::Item> {
        self.item.as_ref()
    }

    fn consume(&mut self) {
        self.item = Source::read(&mut self.source);
    }
}
