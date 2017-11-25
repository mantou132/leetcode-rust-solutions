use super::super::compat::prelude::*;
use std::option;
use super::{Source, Sink};
use super::peek::Peekable;

pub struct TestSource<'a> {
    s: &'a [u8],
}

impl<'a> Source for TestSource<'a> {
    type Item = u8;

    fn read(&mut self) -> Option<u8> {
        match self.s.split_first() {
            option::Option::Some((i,s)) => {
                self.s = s;
                Some(*i)
            },
            option::Option::None => {
                None
            },
        }
    }
}

impl<'a> TestSource<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        TestSource {
            s: s,
        }
    }
}


pub fn new_test_source<'a>(s: &'a [u8]) -> Peekable<TestSource<'a>> {
    Peekable::new(TestSource::new(s))
}


pub struct TestSink<'a> {
    offset: usize,
    s: &'a mut [u8],
}


impl<'a> Sink for TestSink<'a> {
    type Item = u8;

    fn write(&mut self, c: u8) {
        if self.offset == self.s.len() {
            abort!("buffer overflow");
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
