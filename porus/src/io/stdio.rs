use super::peek::Peekable;
use super::super::os::file::{FileSource, FileSink};

pub type Input = Peekable<FileSource>;
pub type Output = FileSink;

pub const fn stdin(buffer: &mut [u8]) -> Peekable<FileSource> {
    Peekable::new(FileSource::new(0, buffer.len() as isize, buffer.as_ptr() as *mut _))
}

pub const fn stdout(buffer: &mut [u8]) -> FileSink {
    FileSink::new(1, buffer.len() as isize, buffer.as_ptr() as *mut _)
}
