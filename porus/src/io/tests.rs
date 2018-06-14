use super::super::iter::{Iter, SliceIter, Peekable, into_iter};
use super::Sink;

pub fn new_test_source<'a>(s: &'a [u8]) -> Peekable<SliceIter<'a, u8>> {
    into_iter(s).peek()
}


pub struct TestSink<'a> {
    offset: usize,
    s: &'a mut [u8],
}


impl<'a> Sink for TestSink<'a> {
    type Item = u8;

    fn write(&mut self, c: u8) {
        if self.offset == self.s.len() {
            panic!("buffer overflow");
        }
        self.s[self.offset] = c;
        self.offset += 1;
    }
}

impl<'a> TestSink<'a> {
    pub fn new(s: &'a mut[u8]) -> Self {
        TestSink {
            offset: 0,
            s: s,
        }
    }
}


pub fn new_test_sink<'a>(s: &'a mut [u8]) -> TestSink<'a> {
    TestSink::new(s)
}
