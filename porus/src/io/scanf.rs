use super::super::iter::Iter;
use super::PeekableSource;

pub trait Converter {
    fn write(&mut self, c: u8);
}


pub fn whitespace<I : Iter<Item=u8>>(s: &mut PeekableSource<I>) {
    while let Some(&c) = s.peek() {
        match c {
            b' ' | b'\t' ... b'\r' => { s.consume(); },
            _ => { break; },
        }
    }
}

pub fn exact<I : Iter<Item=u8>>(s: &mut PeekableSource<I>, c: u8) {
    if let Some(&ch) = s.peek() {
        if c == ch {
            s.consume();
            return;
        }
    }
    panic!("scan error");
}

pub fn character<I : Iter<Item=u8>, C : Converter>(s: &mut PeekableSource<I>, cv: &mut C) {
    if let Some(&c) = s.peek() {
        Converter::write(cv, c);
        s.consume();
        return;
    }
    panic!("scan error");
}

fn is_digit(c: u8, base: u8) -> bool {
    let d =
        match c {
            b'0' ... b'9' => { c - b'0' },
            b'A' ... b'Z' => { c - b'A' + 10u8 },
            b'a' ... b'z' => { c - b'a' + 10u8 },
            _ => { return false; },
        };
    d < base
}

pub fn unsigned<I : Iter<Item=u8>, C: Converter>(s: &mut PeekableSource<I>, cv: &mut C, base: u8) {
    match s.peek() {
        Some(&c) if is_digit(c, base) => {
            Converter::write(cv, c);
            s.consume();

            while let Some(&c) = s.peek() {
                if is_digit(c, base) {
                    Converter::write(cv, c);
                    s.consume();
                } else {
                    break;
                }
            }

            return;
        },
        _ => panic!("scan error"),
    }
}

pub fn signed<I : Iter<Item=u8>, C : Converter>(s: &mut PeekableSource<I>, cv: &mut C, base: u8) {
    match s.peek() {
        Some(&b'-')  => {
            Converter::write(cv, b'-');
            s.consume();
        },
        _ => {},
    }

    unsigned(s, cv, base)
}


pub struct Ignore;

impl Converter for Ignore {
    fn write(&mut self, _: u8) {
    }
}

pub trait CharPattern {
    type Converter: Converter;

    fn converter(self) -> Self::Converter;
}

pub trait UnsignedPattern {
    type Converter: Converter;

    fn converter(self, base: u8) -> Self::Converter;
}

pub trait SignedPattern {
    type Converter: Converter;

    fn converter(self, base: u8) -> Self::Converter;
}


#[cfg(test)]
mod tests {
    use porus_macros::scanf;
    use super::super::super::io;
    use super::super::eof;
    use super::super::tests::new_test_source;

    #[test]
    fn test_whitespace() {
        let source = &mut new_test_source(b"   ");
        scanf!(source, " ");
        assert!(eof(source));
    }

    #[test]
    fn test_exact_match() {
        let source = &mut new_test_source(b"a");
        scanf!(source, "a");
        assert!(eof(source));
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_exact_mismatch() {
        let source = &mut new_test_source(b"b");
        scanf!(source, "a");
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_exact_mismatch_empty() {
        let source = &mut new_test_source(b"");
        scanf!(source, "a");
    }

    #[test]
    fn test_ignore_char_match() {
        let source = &mut new_test_source(b"a");
        scanf!(source, "%*c");
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_ignore_char_mismatch() {
        let source = &mut new_test_source(b"");
        scanf!(source, "%*c");
    }

    #[test]
    fn test_match_char_match() {
        let source = &mut new_test_source(b"a");
        let mut c = 0u8;
        scanf!(source, "%c", &mut c);
        assert!(c == b'a');
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_match_char_mismatch() {
        let source = &mut new_test_source(b"");
        let mut c = 0u8;
        scanf!(source, "%c", &mut c);
    }

    #[test]
    fn test_ignore_unsigned_match() {
        let source = &mut new_test_source(b"a");
        scanf!(source, "%*x");
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_ignore_unsigned_mismatch() {
        let source = &mut new_test_source(b"g");
        scanf!(source, "%*x");
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_ignore_unsigned_mismatch_empty() {
        let source = &mut new_test_source(b"");
        scanf!(source, "%*x");
    }

    #[test]
    fn test_match_unsigned_match() {
        let source = &mut new_test_source(b"a");
        let mut x = 0usize;
        scanf!(source, "%x", &mut x);
        assert!(x == 0xa);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_match_unsigned_mismatch() {
        let source = &mut new_test_source(b"g");
        let mut x = 0usize;
        scanf!(source, "%x", &mut x);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_match_unsigned_mismatch_empty() {
        let source = &mut new_test_source(b"");
        let mut x = 0usize;
        scanf!(source, "%x", &mut x);
    }

    #[test]
    fn test_match_signed_match() {
        let source = &mut new_test_source(b"-123");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
        assert!(x == -123);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_match_signed_mismatch() {
        let source = &mut new_test_source(b"g");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_match_signed_mismatch_empty() {
        let source = &mut new_test_source(b"");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_match_signed_mismatch_sign() {
        let source = &mut new_test_source(b"-g");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
    }

    #[test]
    #[should_panic(expected="scan error")]
    fn test_match_signed_mismatch_sign_empty() {
        let source = &mut new_test_source(b"-");
        let mut x = 0isize;
        scanf!(source, "%d", &mut x);
    }

    #[test]
    fn test_match() {
        let source = &mut new_test_source(b"123 456");
        let mut x = 0isize;
        let mut y = 0isize;
        scanf!(source, " %d %d", &mut x, &mut y);
        assert!(x == 123);
        assert!(y == 456);
    }
}
