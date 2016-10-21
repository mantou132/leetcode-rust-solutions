use super::super::traits;
use super::super::storage::Chunk;
use super::super::libc::{read, write};
use super::super::iter::{peeking, Peekable};

pub struct InputStream {
    fd: i32,
    size: usize,
    offset: usize,
    buffer: Chunk<u8>,
}

impl traits::Iterator for InputStream {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let capacity = self.buffer.capacity();

        if (self.offset == self.size) && (self.size == capacity) {
            self.offset = 0;
            self.size = read(self.fd, self.buffer.as_mut_ptr(), capacity);
        }

        match self.offset < self.size {
            true => {
                let c = self.buffer.read(self.offset);
                self.offset += 1;
                Some(c)
            },
            false =>
                None
        }
    }
}

pub struct OutputStream {
    fd: i32,
    offset: usize,
    buffer: Chunk<u8>,
}

impl traits::OutputStream for OutputStream {
    fn write(&mut self, c: u8) {
        let capacity = self.buffer.capacity();
        if self.offset == capacity {
            write(self.fd, self.buffer.as_ptr(), capacity);
            self.offset = 0;
        }

        self.buffer.write(self.offset, c);
        self.offset += 1;
    }
}

impl Drop for OutputStream {
    fn drop(&mut self) {
        write(self.fd, self.buffer.as_ptr(), self.offset);
    }
}

pub fn input(fd: i32, buffer_size: usize) -> Peekable<InputStream> {
    peeking(
        InputStream {
            fd: fd,
            size: buffer_size,
            offset: buffer_size,
            buffer: Chunk::with_capacity(buffer_size)
        })
}

pub fn output(fd: i32, buffer_size: usize) -> OutputStream {
    OutputStream {
        fd: fd,
        offset: 0,
        buffer: Chunk::with_capacity(buffer_size)
    }
}
