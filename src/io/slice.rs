use super::super::iter::Iterator;
use super::{PeekableSource, Sink};

pub struct SliceSource<'a> {
    offset: usize,
    s: &'a [u8],
}

impl<'a> Iterator for SliceSource<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.offset < self.s.len() {
            let c = self.s[self.offset];
            self.offset += 1;
            Some(c)
        } else {
            None
        }
    }
}

impl<'a> SliceSource<'a> {
    pub fn new(s: &'a [u8]) -> PeekableSource<Self> {
        PeekableSource::new(SliceSource { offset: 0, s })
    }
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
    pub fn new(s: &'a mut [u8]) -> Self {
        SliceSink { offset: 0, s }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
