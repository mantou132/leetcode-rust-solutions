use super::compat::prelude::*;
use std::error::Error;
use std::fmt;
use std::convert::From;

pub trait Source {
    type Item;
    type Error : Error;

    fn read(&mut self) -> Result<Option<Self::Item>,Self::Error>;
}

pub trait PeekableSource {
    type Item;
    type Error : Error;

    fn peek(&mut self) -> Option<&Self::Item>;
    fn consume(&mut self) -> Result<(), Self::Error>;

    fn eof(&mut self) -> bool {
        match self.peek() {
            None => true,
            _ => false,
        }
    }
}

pub trait Sink {
    type Item;
    type Error : Error;

    fn write(&mut self, c: Self::Item) -> Result<(),Self::Error>;
}


#[derive(Debug)]
pub enum ScanError<E: Error> {
    EOF,
    BadInput,
    Error(E)
}

impl<E: Error> fmt::Display for ScanError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScanError::EOF => write!(f, "EOF"),
            ScanError::BadInput => write!(f, "BadInput"),
            ScanError::Error(ref e) => write!(f, "Error({})", e),
        }
    }
}

impl<E: Error> Error for ScanError<E> {
    fn description(&self) -> &str {
        match *self {
            ScanError::EOF => "EOF",
            ScanError::BadInput => "BadInput",
            ScanError::Error(ref e) => Error::description(e),
        }
    }

    fn cause<'a>(&'a self) -> Option<&'a Error> {
        match *self {
            ScanError::EOF => None,
            ScanError::BadInput => None,
            ScanError::Error(ref e) => Some(e),
        }
    }
}

impl<E: Error> From<E> for ScanError<E> {
    fn from(error: E) -> Self {
        ScanError::Error(error)
    }
}

mod peek;
mod file;

mod num;

mod write;
mod print;

pub use self::write::write;
pub use self::print::print;

#[macro_use]
pub mod scanf;

mod stdio;
pub use self::stdio::{stdin, stdout};


pub fn ignore<S: PeekableSource, Fun: Fn(&S::Item) -> bool>(s: &mut S, ignore: Fun) -> Result<(), ScanError<S::Error>> {
    loop {
        match PeekableSource::peek(s) {
            Some(c) if ignore(c) => (),
            _ => break,
        }

        PeekableSource::consume(s)?
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::super::compat::prelude::*;
    use std::error::Error;
    use std::fmt;

    use super::{Source, PeekableSource, Sink, write, print};
    use super::peek::Peekable;

    #[derive(Debug)]
    pub struct EOF;

    impl fmt::Display for EOF {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Error for EOF {
        fn description(&self) -> &str {
            "EOF"
        }

        fn cause(&self) -> Option<&Error> {
            None
        }
    }


    pub struct TestSource<'a> {
        s: &'a [u8],
    }

    impl<'a> Source for TestSource<'a> {
        type Item = u8;
        type Error = EOF;

        fn read(&mut self) -> Result<Option<u8>,Self::Error> {
            match self.s.split_first() {
                Some((i,s)) => {
                    self.s = s;
                    Ok(Some(*i))
                },
                None => {
                    Ok(None)
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


    fn new_test_source<'a>(s: &'a [u8]) -> Peekable<TestSource<'a>> {
        Peekable::new(TestSource::new(s)).unwrap()
    }

    #[test]
    fn test_peekable() {
        let source = &mut new_test_source(b"123");
        assert!(Some(&b'1') == PeekableSource::peek(source));
        assert!(PeekableSource::consume(source).is_ok());
        assert!(Some(&b'2') == PeekableSource::peek(source));
        assert!(PeekableSource::consume(source).is_ok());
        assert!(Some(&b'3') == PeekableSource::peek(source));
        assert!(PeekableSource::consume(source).is_ok());
        assert!(None == PeekableSource::peek(source));
    }

    #[test]
    fn test_scanf_whitespace() {
        let source = &mut new_test_source(b"   ");
        assert!(scanf!(source, " ").is_ok());
        assert!(PeekableSource::eof(source));
    }

    #[test]
    fn test_scanf_exact_match() {
        let source = &mut new_test_source(b"a");
        assert!(scanf!(source, "a").is_ok());
        assert!(PeekableSource::eof(source));
    }

    #[test]
    fn test_scanf_exact_mismatch() {
        let source = &mut new_test_source(b"b");
        assert!(scanf!(source, "a").is_err());
        let source = &mut new_test_source(b"");
        assert!(scanf!(source, "a").is_err());
    }

    #[test]
    fn test_scanf_ignore_char_match() {
        let source = &mut new_test_source(b"a");
        assert!(scanf!(source, "%*c").is_ok());
    }

    #[test]
    fn test_scanf_ignore_char_mismatch() {
        let source = &mut new_test_source(b"");
        assert!(scanf!(source, "%*c").is_err());
    }

    #[test]
    fn test_scanf_match_char_match() {
        let source = &mut new_test_source(b"a");
        let mut c = 0u8;
        assert!(scanf!(source, "%c", &mut c).is_ok());
        assert!(c == b'a');
    }

    #[test]
    fn test_scanf_match_char_mismatch() {
        let source = &mut new_test_source(b"");
        let mut c = 0u8;
        assert!(scanf!(source, "%c", &mut c).is_err());
        assert!(c == 0);
    }

    #[test]
    fn test_scanf_ignore_unsigned_match() {
        let source = &mut new_test_source(b"a");
        assert!(scanf!(source, "%*x").is_ok());
    }

    #[test]
    fn test_scanf_ignore_unsigned_mismatch() {
        let source = &mut new_test_source(b"g");
        assert!(scanf!(source, "%*x").is_err());
    }

    #[test]
    fn test_scanf_match_unsigned_match() {
        let source = &mut new_test_source(b"a");
        let mut x = 0usize;
        assert!(scanf!(source, "%x", &mut x).is_ok());
        assert!(x == 0xa);
    }

    #[test]
    fn test_scanf_match_unsigned_mismatch() {
        let source = &mut new_test_source(b"g");
        let mut x = 0usize;
        assert!(scanf!(source, "%x", &mut x).is_err());
        assert!(x == 0);
    }

    #[test]
    fn test_scanf_match_signed_match() {
        let source = &mut new_test_source(b"-123");
        let mut x = 0isize;
        assert!(scanf!(source, "%d", &mut x).is_ok());
        assert!(x == -123);
    }

    #[test]
    fn test_scanf_match_signed_mismatch() {
        let source = &mut new_test_source(b"-");
        let mut x = 0isize;
        assert!(scanf!(source, "%d", &mut x).is_err());
        assert!(x == 0);

        let source = &mut new_test_source(b"");
        let mut x = 0isize;
        assert!(scanf!(source, "%d", &mut x).is_err());
        assert!(x == 0);
    }

    #[test]
    fn test_scanf_match() {
        let source = &mut new_test_source(b"123 456");
        let mut x = 0isize;
        let mut y = 0isize;
        assert!(scanf!(source, " %d %d", &mut x, &mut y).is_ok());
        assert!(x == 123);
        assert!(y == 456);
    }


    pub struct TestSink<'a> {
        offset: usize,
        s: &'a mut [u8],
    }


    #[derive(Debug)]
    pub struct BufferOverflow;

    impl fmt::Display for BufferOverflow {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Error for BufferOverflow {
        fn description(&self) -> &str {
            "BufferOverflow"
        }

        fn cause(&self) -> Option<&Error> {
            None
        }
    }

    impl<'a> Sink for TestSink<'a> {
        type Item = u8;
        type Error = BufferOverflow;

        fn write(&mut self, c: u8) -> Result<(),Self::Error> {
            if self.offset == self.s.len() {
                Err(BufferOverflow)
            } else {
                self.s[self.offset] = c;
                self.offset += 1;
                Ok(())
            }
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

    #[test]
    fn test_write_unsigned() {
        let array = &mut [0;1];
        {
            let sink = &mut TestSink::new(array);
            assert!(write(sink, 0u8).is_ok());
        }
        assert!(array == b"0");

        let array = &mut [0;3];
        {
            let sink = &mut TestSink::new(array);
            assert!(write(sink, 123u8).is_ok());
        }
        assert!(array == b"123");
    }

    #[test]
    fn test_write_signed() {
        let array = &mut [0;1];
        {
            let sink = &mut TestSink::new(array);
            assert!(write(sink, 0i8).is_ok());
        }
        assert!(array == b"0");

        let array = &mut [0;4];
        {
            let sink = &mut TestSink::new(array);
            assert!(write(sink, -123i8).is_ok());
        }
        assert!(array == b"-123");
    }

    #[test]
    fn test_write_overflow() {
        let array = &mut [0;1];
        {
            let sink = &mut TestSink::new(array);
            assert!(write(sink, 123u8).is_err());
        }
    }

    #[test]
    fn test_print() {
        let array = &mut [0;7];
        {
            let sink = &mut TestSink::new(array);
            assert!(print(sink, (123," ",456)).is_ok());
        }
        assert!(array == b"123 456");
    }
}
