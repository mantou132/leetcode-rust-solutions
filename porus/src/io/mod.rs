use super::iter::{Iter, Peekable};

pub trait Source : Iter<Item=u8> {}

impl<T : Iter<Item=u8>> Source for T {}

pub type PeekableSource<I> = Peekable<I>;

pub fn eof<I : Iter<Item=u8>>(source : &mut PeekableSource<I>) -> bool {
    match source.peek() {
        None => true,
        _ => false,
    }
}

pub trait Sink {
    fn write(&mut self, c: u8);
}

pub mod slice;

mod int;

pub mod printf;

pub mod read;
pub use self::read::{fread, Whitespace};

pub mod stdio;
pub use self::stdio::{stdin, stdout};
