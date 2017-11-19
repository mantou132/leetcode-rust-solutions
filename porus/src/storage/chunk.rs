use std::mem::size_of;
use std::ptr::{read, write};

use super::super::os::{malloc, realloc, free};


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

    pub fn as_ptr(&mut self) -> *const T {
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

    fn get_ptr(&self, index: isize) -> Option<*const T> {
       if (0 <= index) && (index < self.capacity) {
           Some(unsafe {self.data.offset(index)})
       } else {
           None
        }
    }

    fn get_mut_ptr(&self, index: isize) -> Option<*mut T> {
        if (0 <= index) && (index < self.capacity) {
            Some(unsafe {self.data.offset(index)})
        } else {
            None
        }
    }

    pub fn read(&mut self, index: isize) -> Option<T> {
        self.get_ptr(index).map(|p| unsafe { read(p) })
    }

    pub fn write(&mut self, index: isize, item: T) {
        if let Some(p) = self.get_mut_ptr(index) {
            unsafe { write(p, item) };
        }
    }

    pub fn get(&mut self, index: isize) -> Option<&T> {
        self.get_ptr(index).map(|p| unsafe{&*p})
    }

    pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        self.get_mut_ptr(index).map(|p| unsafe{&mut *p})
    }
}


impl<T> Drop for Chunk<T>{
    fn drop(&mut self){
        free(self.data as *mut _)
    }
}
