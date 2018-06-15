use super::super::iter::{Iter, IntoIter, Peekable, into_iter};
use super::Sink;

pub fn new_slice_source<'a>(s: &'a [u8]) -> Peekable<<&'a [u8] as IntoIter>::Iter> {
    into_iter(s).peek()
}

pub struct SliceSink<'a> {
    offset: usize,
    s: &'a mut [u8],
}


impl<'a> Sink for SliceSink<'a> {
    fn write(&mut self, c: u8) {
        if self.offset == self.s.len() {
            panic!("buffer overflow");
        }
        self.s[self.offset] = c;
        self.offset += 1;
    }
}

impl<'a> SliceSink<'a> {
    pub fn new(s: &'a mut[u8]) -> Self {
        SliceSink {
            offset: 0,
            s: s,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
