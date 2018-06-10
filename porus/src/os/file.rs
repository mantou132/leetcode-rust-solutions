use super::super::compat::prelude::*;
use super::super::ptr;
use super::super::io::{Source, Sink};
use super::libc;
use super::OSError;


pub fn read(fd: i32, buf: *mut u8, count: usize) -> Result<usize, OSError> {
    let mut length = 0;
    let mut ptr = buf;

    while length < count {
        let size =
            unsafe {
                libc::read(fd, ptr, count-length)
            };

        if size < 0 {
            return libc::get_error();
        }
        if size == 0 {
            break;
        }

        length += size as usize;

        unsafe {
            ptr = ptr.offset(size);
        }
    }

    Ok(length)
}

pub fn write(fd: i32, buf: *const u8, count: usize) -> Result<(), OSError> {
    let mut written = 0;
    let mut ptr = buf;
    while written < count {
        let size =
            unsafe {
                libc::write(fd, ptr, count-written)
            };

        if size < 0 {
            return libc::get_error();
        }

        written += size as usize;

        unsafe {
            ptr = ptr.offset(size);
        }
    }

    Ok(())
}

pub struct FileSource {
    fd: i32,
    size: isize,
    offset: isize,
    capacity: isize,
    buffer: *mut u8,
}

impl FileSource {
    pub fn new(fd: i32, size: isize, buffer: *mut u8) -> Self {
        FileSource {
            fd: fd,
            size: size,
            offset: size,
            capacity: size,
            buffer: buffer,
        }
    }
}

impl Source for FileSource {
    type Item = u8;

    fn read(&mut self) -> Option<u8> {
        if (self.offset == self.size) && (self.size == self.capacity) {
            self.offset = 0;
            self.size = read(self.fd, self.buffer, self.capacity as usize).unwrap() as isize;
        }

        if self.offset < self.size {
            let c = ptr::read(self.buffer, self.offset);
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
    capacity: isize,
    buffer: *mut u8,
}

impl FileSink {
    pub fn new(fd: i32, size: isize, buffer: *mut u8) -> Self {
        FileSink{
            fd: fd,
            offset: 0,
            capacity: size,
            buffer: buffer,
        }
    }
}

impl Sink for FileSink {
    type Item = u8;

    fn write(&mut self, c: u8) {
        if self.offset == self.capacity {
            write(self.fd, self.buffer, self.capacity as usize).unwrap();
            self.offset = 0;
        }

        ptr::write(self.buffer, self.offset, c);
        self.offset += 1;
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        write(self.fd, self.buffer, self.offset as usize).unwrap();
    }
}
