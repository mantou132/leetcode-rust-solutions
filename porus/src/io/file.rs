use super::{Source, Sink};
use super::super::os::{read, write};
use super::super::storage::Chunk;


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
        let capacity = self.buffer.capacity();

        if (self.offset == self.size) && (self.size == capacity) {
            self.offset = 0;
            self.size = read(self.fd, self.buffer.as_mut_ptr(), capacity as usize).unwrap() as isize;
        }

        let c = self.buffer.read(self.offset);
        if self.offset < self.size {
            self.offset += 1;
        }
        c
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
            write(self.fd, self.buffer.as_ptr(), capacity as usize).unwrap();
            self.offset = 0;
        }

        self.buffer.write(self.offset, c);
        self.offset += 1;
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        write(self.fd, self.buffer.as_ptr(), self.offset as usize).unwrap();
    }
}
