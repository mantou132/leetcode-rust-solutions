use std::mem::size_of;
use std::ptr::{read, write, copy};
use super::libc::{malloc, realloc, free};


pub struct Chunk<T> {
    capacity: isize,
    data: *mut T,
}


impl<T> Chunk<T> {

    pub fn new(mut capacity: isize) -> Self {
        if capacity < 0 {
            capacity = 0;
        }

        let size = size_of::<T>() * (capacity as usize);

        Chunk {
            capacity: capacity,
            data: malloc(size).unwrap() as *mut _,
        }
    }

    pub fn capacity(&self) -> isize {
        self.capacity
    }

    pub fn as_ptr(&self) -> *const T {
        self.data
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }

    pub fn resize(&mut self, mut capacity: isize) {
        if capacity < 0 {
            capacity = 0;
        }

        let size = size_of::<T>() * (capacity as usize);
        self.data = realloc(self.data as *mut _, size).unwrap() as *mut _;
        self.capacity = capacity;
    }

    pub fn copy(&mut self, src: isize, dst: isize, count: isize) {
        unsafe {
            copy(self.data.offset(src), self.data.offset(dst), count as usize);
        }
    }

    pub fn read(&mut self, index: isize) -> T {
        unsafe { read(self.data.offset(index)) }
    }

    pub fn write(&mut self, index: isize, item: T) {
        unsafe { write(self.data.offset(index), item) }
    }

    pub fn get(&self, index: isize) -> &T {
        unsafe { &*self.data.offset(index) }
    }

    pub fn get_mut(&mut self, index: isize) -> &mut T {
        unsafe { &mut *self.data.offset(index) }
    }
}


impl<T> Drop for Chunk<T> {
    fn drop(&mut self){
        free(self.data as *mut _)
    }
}
