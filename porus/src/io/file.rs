use super::super::compat::prelude::*;
use super::{Source, Sink};
use super::super::os::{read, write};
use super::super::chunk::Chunk;


pub struct FileSource {
    fd: i32,
    size: isize,
    offset: isize,
    buffer: Chunk<u8>,
}

impl FileSource {
    pub fn new(fd: i32, buffer_size: isize) -> Self {
        FileSource {
            fd: fd,
            size: buffer_size,
            offset: buffer_size,
            buffer: Chunk::new(buffer_size)
        }
    }
}

impl Source for FileSource {
    type Item = u8;

    fn read(&mut self) -> Option<u8> {
        let capacity = Chunk::capacity(&self.buffer);

        if (self.offset == self.size) && (self.size == capacity) {
            self.offset = 0;
            self.size = read(self.fd, Chunk::as_mut_ptr(&mut self.buffer), capacity as usize).unwrap() as isize;
        }

        if self.offset < self.size {
            let c = Chunk::read(&mut self.buffer, self.offset);
            self.offset += 1;
            Some(c)
        } else {
            None
        }
    }
}


pub struct FileSink {
    fd: i32,
    offset: isize,
    buffer: Chunk<u8>,
}

impl FileSink {
    pub fn new(fd: i32, buffer_size: isize) -> Self {
        FileSink{
            fd: fd,
            offset: 0,
            buffer: Chunk::new(buffer_size)
        }
    }
}

impl Sink for FileSink {
    type Item = u8;

    fn write(&mut self, c: u8) {
        let capacity = self.buffer.capacity();
        if self.offset == capacity {
            write(self.fd, Chunk::as_ptr(&self.buffer), capacity as usize).unwrap();
            self.offset = 0;
        }

        Chunk::write(&mut self.buffer, self.offset, c);
        self.offset += 1;
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        write(self.fd, Chunk::as_ptr(&self.buffer), self.offset as usize).unwrap();
    }
}
