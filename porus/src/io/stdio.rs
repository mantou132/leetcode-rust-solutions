use super::peek::Peekable;
use super::file::{FileSource, FileSink};

pub fn stdin() -> Peekable<FileSource> {
    Peekable::new(FileSource::new(0, 1024))
}

pub fn stdout() -> FileSink {
    FileSink::new(1, 1024)
}
