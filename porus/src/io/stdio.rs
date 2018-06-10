use super::peek::Peekable;
use super::super::os::file::{FileSource, FileSink};
use super::super::alloc::allocate;
use super::super::os::OSAllocator;

pub fn stdin() -> Peekable<FileSource> {
    Peekable::new(FileSource::new(0, 1024, allocate(&mut OSAllocator::new(), 1024)))
}

pub fn stdout() -> FileSink {
    FileSink::new(1, 1024, allocate(&mut OSAllocator::new(), 1024))
}
