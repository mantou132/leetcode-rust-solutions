use super::super::traits::InputStream;
use super::ignore;
use super::num::{read_unsigned, read_signed};

pub trait Read {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self;
}


impl Read for u8 {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_unsigned(stream)
    }
}

impl Read for u16 {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_unsigned(stream)
    }
}

impl Read for u32 {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_unsigned(stream)
    }
}

impl Read for u64 {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_unsigned(stream)
    }
}

impl Read for usize {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_unsigned(stream)
    }
}


impl Read for i8 {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_signed(stream)
    }
}

impl Read for i16 {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_signed(stream)
    }
}

impl Read for i32 {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_signed(stream)
    }
}

impl Read for i64 {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_signed(stream)
    }
}

impl Read for isize {
    fn read<Stream: InputStream>(stream: &mut Stream) -> Self {
        read_signed(stream)
    }
}

pub trait Reader {
    fn read<T: Read>(&mut self) -> T;
}


pub struct InputStreamReader<Stream: InputStream, Fun: Fn(u8) -> bool> {
    stream: Stream,
    ignore: Fun,
}

impl<Stream: InputStream, Fun: Fn(u8) -> bool> InputStreamReader<Stream, Fun> {
    pub fn new(stream: Stream, ignore: Fun) -> Self {
        Self {stream: stream, ignore: ignore}
    }
}


impl<Stream: InputStream, Fun: Fn(u8) -> bool> Reader for InputStreamReader<Stream, Fun> {
    fn read<T: Read>(&mut self) -> T {
        ignore(&mut self.stream, &self.ignore);
        Read::read(&mut self.stream)
    }
}

pub fn read<T: Read, R: Reader>(reader: &mut R) -> T {
    Reader::read(reader)
}
