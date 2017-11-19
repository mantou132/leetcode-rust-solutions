pub trait Source {
    type Item;

    fn read(&mut self) -> Option<Self::Item>;
}

pub trait PeekableSource {
    type Item;

    fn peek(&mut self) -> Option<&Self::Item>;
    fn consume(&mut self);

    fn eof(&mut self) -> bool {
        match self.peek() {
            None => true,
            _ => false,
        }
    }
}

pub trait Sink {
    type Item;

    fn write(&mut self, c: Self::Item);
}

mod peek;
mod file;

mod num;

#[macro_use]
pub mod scanf;
#[macro_use]
pub mod printf;

mod stdio;
pub use self::stdio::{stdin, stdout};


#[cfg(test)]
mod tests {
    use super::{Source, PeekableSource, Sink};
    use super::peek::Peekable;

    pub struct TestSource<'a> {
        s: &'a [u8],
    }

    impl<'a> Source for TestSource<'a> {
        type Item = u8;

        fn read(&mut self) -> Option<u8> {
            match self.s.split_first() {
                Some((i,s)) => {
                    self.s = s;
                    Some(*i)
                },
                None => {
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


    fn new_test_source<'a>(s: &'a [u8]) -> Peekable<TestSource<'a>> {
        Peekable::new(TestSource::new(s))
    }

    #[test]
    fn test_peekable() {
        let source = &mut new_test_source(b"123");
        assert!(Some(&b'1') == PeekableSource::peek(source));
        PeekableSource::consume(source);
        assert!(Some(&b'2') == PeekableSource::peek(source));
        PeekableSource::consume(source);
        assert!(Some(&b'3') == PeekableSource::peek(source));
        PeekableSource::consume(source);
        assert!(None == PeekableSource::peek(source));
    }

    #[test]
    fn test_scanf_whitespace() {
        let source = &mut new_test_source(b"   ");
        scanf!(source, " ");
        assert!(PeekableSource::eof(source));
    }

    #[test]
    fn test_scanf_exact_match() {
        let source = &mut new_test_source(b"a");
        scanf!(source, "a");
        assert!(PeekableSource::eof(source));
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_exact_mismatch() {
        let source = &mut new_test_source(b"b");
        scanf!(source, "a");
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_exact_mismatch_empty() {
        let source = &mut new_test_source(b"");
        scanf!(source, "a");
    }

    #[test]
    fn test_scanf_ignore_char_match() {
        let source = &mut new_test_source(b"a");
        scanf!(source, "%*c");
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_ignore_char_mismatch() {
        let source = &mut new_test_source(b"");
        scanf!(source, "%*c");
    }

    #[test]
    fn test_scanf_match_char_match() {
        let source = &mut new_test_source(b"a");
        let mut c = 0u8;
        scanf!(source, "%c", &mut c);
        assert!(c == b'a');
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_match_char_mismatch() {
        let source = &mut new_test_source(b"");
        let mut c = 0u8;
        scanf!(source, "%c", &mut c);
    }

    #[test]
    fn test_scanf_ignore_unsigned_match() {
        let source = &mut new_test_source(b"a");
        scanf!(source, "%*x");
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_ignore_unsigned_mismatch() {
        let source = &mut new_test_source(b"g");
        scanf!(source, "%*x");
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_ignore_unsigned_mismatch_empty() {
        let source = &mut new_test_source(b"");
        scanf!(source, "%*x");
    }

    #[test]
    fn test_scanf_match_unsigned_match() {
        let source = &mut new_test_source(b"a");
        let mut x = 0usize;
        scanf!(source, "%x", &mut x);
        assert!(x == 0xa);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_match_unsigned_mismatch() {
        let source = &mut new_test_source(b"g");
        let mut x = 0usize;
        scanf!(source, "%x", &mut x);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_match_unsigned_mismatch_empty() {
        let source = &mut new_test_source(b"");
        let mut x = 0usize;
        scanf!(source, "%x", &mut x);
    }

    #[test]
    fn test_scanf_match_signed_match() {
        let source = &mut new_test_source(b"-123");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
        assert!(x == -123);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_match_signed_mismatch() {
        let source = &mut new_test_source(b"g");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_match_signed_mismatch_empty() {
        let source = &mut new_test_source(b"");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_match_signed_mismatch_sign() {
        let source = &mut new_test_source(b"-g");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_scanf_match_signed_mismatch_sign_empty() {
        let source = &mut new_test_source(b"-");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
    }

    #[test]
    fn test_scanf_match() {
        let source = &mut new_test_source(b"123 456");
        let mut x = 0isize;
        let mut y = 0isize;
        scanf!(source, " %d %d", &mut x, &mut y);
        assert!(x == 123);
        assert!(y == 456);
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

    #[test]
    fn test_print_char() {
        let array = &mut [0;1];
        {
            let sink = &mut TestSink::new(array);
            printf!(sink, "%c", b'0');
        }
        assert!(array == b"0");
    }

    #[test]
    fn test_print_unsigned() {
        let array = &mut [0;1];
        {
            let sink = &mut TestSink::new(array);
            printf!(sink, "%d", 0u8);
        }
        assert!(array == b"0");

        let array = &mut [0;3];
        {
            let sink = &mut TestSink::new(array);
            printf!(sink, "%d", 123u8);
        }
        assert!(array == b"123");
    }

    #[test]
    fn test_print_signed() {
        let array = &mut [0;1];
        {
            let sink = &mut TestSink::new(array);
            printf!(sink, "%d", 0i8);
        }
        assert!(array == b"0");

        let array = &mut [0;3];
        {
            let sink = &mut TestSink::new(array);
            printf!(sink, "%d", 123i8);
        }
        assert!(array == b"123");

        let array = &mut [0;4];
        {
            let sink = &mut TestSink::new(array);
            printf!(sink, "%d", -123i8);
        }
        assert!(array == b"-123");
    }

    #[test]
    fn test_print_string() {
        let array = &mut [0;5];
        {
            let sink = &mut TestSink::new(array);
            printf!(sink, "%s", "hello");
        }
        assert!(array == b"hello");
    }

    #[test]
    #[should_panic(expected="buffer overflow")]
    fn test_print_overflow() {
        let array = &mut [0;1];
        {
            let sink = &mut TestSink::new(array);
            printf!(sink, "%d", 123u8);
        }
    }

    #[test]
    fn test_print() {
        let array = &mut [0;7];
        {
            let sink = &mut TestSink::new(array);
            printf!(sink, "%d %d", 123, 456);
        }
        assert!(array == b"123 456");
    }
}
