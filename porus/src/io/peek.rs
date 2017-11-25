use super::super::compat::prelude::*;
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

#[cfg(test)]
mod tests {
    use super::super::super::compat::prelude::*;
    use super::super::PeekableSource;
    use super::super::tests::new_test_source;

    #[test]
    fn test() {
        let source = &mut new_test_source(b"123");
        assert!(Some(&b'1') == PeekableSource::peek(source));
        PeekableSource::consume(source);
        assert!(Some(&b'2') == PeekableSource::peek(source));
        PeekableSource::consume(source);
        assert!(Some(&b'3') == PeekableSource::peek(source));
        PeekableSource::consume(source);
        assert!(None == PeekableSource::peek(source));
        assert!(PeekableSource::eof(source));
    }
}
