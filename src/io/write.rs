use super::super::traits::OutputStream;
use super::num::{write_unsigned, write_signed};


pub trait Write {
    fn write<Stream: OutputStream>(stream: &mut Stream, Self);
}

pub fn write<T: Write, Stream: OutputStream>(stream: &mut Stream, x: T) {
    Write::write(stream, x)
}


impl Write for u8 {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_unsigned(stream, x)
    }
}

impl Write for u16 {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_unsigned(stream, x)
    }
}

impl Write for u32 {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_unsigned(stream, x)
    }
}

impl Write for u64 {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_unsigned(stream, x)
    }
}

impl Write for usize {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_unsigned(stream, x)
    }
}

impl Write for i8 {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_signed(stream, x)
    }
}

impl Write for i16 {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_signed(stream, x)
    }
}

impl Write for i32 {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_signed(stream, x)
    }
}

impl Write for i64 {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_signed(stream, x)
    }
}

impl Write for isize {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write_signed(stream, x)
    }
}


impl<'a> Write for &'a[u8] {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        for c in x {
            OutputStream::write_char(stream, *c);
        }
    }
}

impl<'a> Write for &'a str {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write(stream, x.as_bytes());
    }
}

pub use super::super::string::String;

impl<'a> Write for &'a String {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write(stream, x.as_bytes());
    }
}


impl Write for () {
    fn write<Stream: OutputStream>(_: &mut Stream, _: Self) {
    }
}

impl<A:Write> Write for (A,) {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write(stream, x.0);
    }
}


impl<A:Write, B:Write> Write for (A,B) {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write(stream, x.0);
        write(stream, x.1);
    }
}

impl<A:Write, B:Write, C:Write> Write for (A,B,C) {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write(stream, x.0);
        write(stream, x.1);
        write(stream, x.2);
    }
}

impl<A:Write, B:Write, C:Write, D:Write> Write for (A,B,C,D) {
    fn write<Stream: OutputStream>(stream: &mut Stream, x: Self) {
        write(stream, x.0);
        write(stream, x.1);
        write(stream, x.2);
        write(stream, x.3);
    }
}
