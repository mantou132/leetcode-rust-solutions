use super::iter::Iterator;


pub trait Source : Iterator<Item=u8> {}

impl<T : Iterator<Item=u8>> Source for T {}

pub struct PeekableSource<S : Source> {
    source: S,
    peeked: Option<Option<S::Item>>,
}

impl<S : Source> PeekableSource<S> {
    pub const fn new(s: S) -> Self {
        PeekableSource {
            source: s,
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&S::Item> {
        if let None = self.peeked {
            self.consume();
        }

        if let Some(ref x) = self.peeked {
            return x.as_ref();
        }

        unreachable!();
    }

    pub fn consume(&mut self) {
        self.peeked = Some(Iterator::next(&mut self.source));
    }

    pub fn eof(&mut self) -> bool {
        match self.peek() {
            None => true,
            _ => false,
        }
    }
}

pub trait Sink {
    fn write(&mut self, c: u8);
}

pub mod slice;
pub mod read;
pub mod write;
pub mod stdio;
