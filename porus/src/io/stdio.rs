use super::super::compat::prelude::*;
use super::super::os::OSError;
use super::peek::Peekable;
use super::file::{FileSource, FileSink};

pub fn stdin() -> Result<Peekable<FileSource>, OSError> {
    Ok(Peekable::new(FileSource::new(0, 1024)?)?)
}

pub fn stdout() -> Result<FileSink, OSError> {
    FileSink::new(1, 1024)
}
