use super::Sink;

pub fn write_char<S: Sink<Item=u8>>(s: &mut S, c: u8) -> &mut S {
    Sink::write(s, c);
    s
}

pub fn write_string<S: Sink<Item=u8>, T: AsRef<[u8]>>(s: &mut S, t: T) -> &mut S {
    for c in AsRef::<[u8]>::as_ref(&t) {
        Sink::write(s, *c);
    }
    s
}

pub trait IntField : Sized {
    type Converter: AsRef<[u8]>;

    fn converter(self, u8) -> Self::Converter;
}

#[cfg(test)]
mod tests {
    use porus_macros::printf;
    use super::super::super::io;
    use super::super::tests::new_test_sink;

    #[test]
    fn test_char() {
        let array = &mut [0;1];
        {
            let sink = &mut new_test_sink(array);
            printf!(sink, "%c", b'0');
        }
        assert!(array == b"0");
    }

    #[test]
    fn test_unsigned() {
        let array = &mut [0;1];
        {
            let sink = &mut new_test_sink(array);
            printf!(sink, "%d", 0u8);
        }
        assert!(array == b"0");

        let array = &mut [0;3];
        {
            let sink = &mut new_test_sink(array);
            printf!(sink, "%d", 123u8);
        }
        assert!(array == b"123");
    }

    #[test]
    fn test_signed() {
        let array = &mut [0;1];
        {
            let sink = &mut new_test_sink(array);
            printf!(sink, "%d", 0i8);
        }
        assert!(array == b"0");

        let array = &mut [0;3];
        {
            let sink = &mut new_test_sink(array);
            printf!(sink, "%d", 123i8);
        }
        assert!(array == b"123");

        let array = &mut [0;4];
        {
            let sink = &mut new_test_sink(array);
            printf!(sink, "%d", -123i8);
        }
        assert!(array == b"-123");
    }

    #[test]
    fn test_string() {
        let array = &mut [0;5];
        {
            let sink = &mut new_test_sink(array);
            printf!(sink, "%s", "hello");
        }
        assert!(array == b"hello");
    }

    #[test]
    #[should_panic(expected="buffer overflow")]
    fn test_overflow() {
        let array = &mut [0;1];
        {
            let sink = &mut new_test_sink(array);
            printf!(sink, "%d", 123u8);
        }
    }

    #[test]
    fn test_print() {
        let array = &mut [0;7];
        {
            let sink = &mut new_test_sink(array);
            printf!(sink, "%d %d", 123, 456);
        }
        assert!(array == b"123 456");
    }
}
