use super::traits::InputStream;
use super::iter::Peekable;

pub mod file;

mod num;

mod read;
pub use self::read::{read, Read};
use self::read::InputStreamReader;

mod write;
pub use self::write::write;

pub use super::string::{read_string, read_string_until};


pub fn eof<Stream: InputStream>(stream: &mut Stream) -> bool {
    match stream.peek() {
        None => true,
        _ => false,
    }
}

pub fn ignore<'a, Stream: InputStream, Fun: Fn(u8) -> bool>(stream: &mut Stream, ignore: &'a Fun) {
    loop {
        match stream.peek() {
            Some(&c) if ignore(c) => (),
            _ => break,
        }

        stream.consume()
    }
}

pub fn stdin<Fun: Fn(u8) -> bool>(buffer_size: usize, ignore: Fun) -> InputStreamReader<Peekable<file::InputStream>,Fun> {
    read::InputStreamReader::new(file::input(0, buffer_size), ignore)
}

pub fn stdout(buffer_size: usize) -> file::OutputStream {
    file::output(1, buffer_size)
}
