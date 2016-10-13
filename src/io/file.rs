use super::super::traits::iter::Iterator;
use super::super::traits::io::OutputStream as OutputStreamTrait;
use super::super::mem::array::Array;
use super::super::libc::{read, write};
use super::super::iter::{peeking, Peekable};

pub struct InputStream {
    fd: i32,
    size: usize,
    offset: usize,
    data: Array<u8>,
}

impl Iterator for InputStream {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let capacity = self.data.capacity();

        if (self.offset == self.size) && (self.size == capacity) {
            self.offset = 0;
            self.size = read(self.fd, self.data.as_mut_ptr(), capacity);
        }

        match self.offset < self.size {
            true => {
                let c = self.data.read(self.offset);
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
    data: Array<u8>,
}

impl OutputStreamTrait for OutputStream {
    fn write(&mut self, c: u8) {
        let capacity = self.data.capacity();
        if self.offset == capacity {
            write(self.fd, self.data.as_ptr(), capacity);
            self.offset = 0;
        }

        self.data.write(self.offset, c);
        self.offset += 1;
    }
}

impl Drop for OutputStream {
    fn drop(&mut self) {
        write(self.fd, self.data.as_ptr(), self.offset);
    }
}


#[cfg_attr(not(debug_assertions), inline(always))]
pub fn input(fd: i32, buffer_size: usize) -> Peekable<InputStream> {
    peeking(
        InputStream {
            fd: fd,
            size: buffer_size,
            offset: buffer_size,
            data: Array::new(buffer_size)
        })
}

#[cfg_attr(not(debug_assertions), inline(always))]
pub fn output(fd: i32, buffer_size: usize) -> OutputStream {
    OutputStream {
        fd: fd,
        offset: 0,
        data: Array::new(buffer_size)
    }
}
