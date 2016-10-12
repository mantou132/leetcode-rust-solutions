use super::iter::PeekingIterator;

pub trait InputStream : PeekingIterator<Item=u8> {
}

impl<T: PeekingIterator<Item=u8>> InputStream for T {
}

pub trait OutputStream {
    fn write(&mut self, c: u8);
}
