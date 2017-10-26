use super::super::compat::prelude::*;
use super::{Source, Sink};
use super::super::os::{OSError, read, write};
use super::super::storage::Chunk;


pub struct FileSource {
    fd: i32,
    size: usize,
    offset: usize,
    buffer: Chunk<u8>,
}

impl FileSource {
    pub fn new(fd: i32, buffer_size: usize) -> Result<Self, OSError> {
        Ok(FileSource{
            fd: fd,
            size: buffer_size,
            offset: buffer_size,
            buffer: Chunk::new(buffer_size)?
        })
    }
}

impl Source for FileSource {
    type Item = u8;
    type Error = OSError;

    fn read(&mut self) -> Result<Option<u8>, OSError> {
        let capacity = self.buffer.capacity();

        if (self.offset == self.size) && (self.size == capacity) {
            self.offset = 0;
            self.size = read(self.fd, self.buffer.as_mut_ptr(), capacity)?;
        }

        match self.offset < self.size {
            true => {
                let c = self.buffer.read(self.offset).unwrap();
                self.offset += 1;
                Ok(Some(c))
            },
            false =>
                Ok(None)
        }
    }
}


pub struct FileSink {
    fd: i32,
    offset: usize,
    buffer: Chunk<u8>,
}

impl FileSink {
    pub fn new(fd: i32, buffer_size: usize) -> Result<Self, OSError> {
        Ok(FileSink{
            fd: fd,
            offset: 0,
            buffer: Chunk::new(buffer_size)?
        })
    }
}

impl Sink for FileSink {
    type Item = u8;
    type Error = OSError;

    fn write(&mut self, c: u8) -> Result<(),OSError> {
        let capacity = self.buffer.capacity();
        if self.offset == capacity {
            write(self.fd, self.buffer.as_ptr(), capacity)?;
            self.offset = 0;
        }

        self.buffer.write(self.offset, c).unwrap();
        self.offset += 1;
        Ok(())
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        write(self.fd, self.buffer.as_ptr(), self.offset).unwrap();
    }
}
