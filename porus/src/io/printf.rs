use super::Sink;
pub use porus_macros::printf_impl;

#[macro_export]
macro_rules! printf {
    ($f:expr, $fmt:expr $(, $arg:expr)*) => (
        $crate::io::printf::printf_impl!($crate, $f, $fmt $(, $arg)*)
    )
}

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
