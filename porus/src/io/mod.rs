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

mod int;

#[macro_use]
pub mod scanf;
#[macro_use]
pub mod printf;

mod stdio;
pub use self::stdio::{stdin, stdout};

#[cfg(test)]
mod tests;
