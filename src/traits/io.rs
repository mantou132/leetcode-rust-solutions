use super::iter::PeekingIterator;


pub trait InputStream : PeekingIterator<Item=u8> {
}

pub trait OutputStream {
    fn write_char(&mut self, c: u8);
}
