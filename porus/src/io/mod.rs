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
    type Item;

    fn write(&mut self, c: Self::Item);
}

mod int;

pub mod scanf;
pub mod printf;

pub mod stdio;
pub use self::stdio::{stdin, stdout};

#[cfg(test)]
mod tests;
