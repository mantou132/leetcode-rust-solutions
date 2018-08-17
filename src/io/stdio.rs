use super::super::os::file::{FileSink, FileSource};
use super::PeekableSource;

pub type Input = PeekableSource<FileSource>;
pub type Output = FileSink;

pub const fn stdin(buffer: &mut [u8]) -> PeekableSource<FileSource> {
    PeekableSource::new(FileSource::new(
        0,
        buffer.len() as isize,
        buffer.as_ptr() as *mut _,
    ))
}

pub const fn stdout(buffer: &mut [u8]) -> FileSink {
    FileSink::new(1, buffer.len() as isize, buffer.as_ptr() as *mut _)
}
