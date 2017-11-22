use std::ptr::{read, write};
use super::super::os::{malloc, realloc, free};


pub struct RawSystemChunk {
    capacity: isize,
    data: *mut u8,
}


impl RawSystemChunk {

    pub fn new(mut capacity: isize) -> Self {
        if capacity < 0 {
            capacity = 0;
        }

        RawSystemChunk {
            capacity: capacity,
            data: malloc(capacity as usize).unwrap(),
        }
    }

    pub fn capacity(&self) -> isize {
        self.capacity
    }

    pub fn as_ptr<T>(&self) -> *const T {
        self.data as *const _
    }

    pub fn as_mut_ptr<T>(&mut self) -> *mut T {
        self.data as *mut _
    }

    pub fn resize(&mut self, mut capacity: isize) {
        if capacity < 0 {
            capacity = 0;
        }

        self.data = realloc(self.data, capacity as usize).unwrap();
        self.capacity = capacity;
    }

    fn get_ptr<T>(&self, index: isize) -> *const T {
        unsafe {self.data.offset(index) as *const _}
    }

    fn get_mut_ptr<T>(&mut self, index: isize) -> *mut T {
        unsafe {self.data.offset(index) as *mut _}
    }

    pub fn read<T>(&mut self, index: isize) -> T {
        unsafe { read(self.get_ptr(index)) }
    }

    pub fn write<T>(&mut self, index: isize, item: T) {
        unsafe { write(self.get_mut_ptr(index), item) }
    }

    pub fn get<T>(&self, index: isize) -> &T {
        unsafe { &*self.get_ptr(index) }
    }

    pub fn get_mut<T>(&mut self, index: isize) -> &mut T {
        unsafe { &mut *self.get_mut_ptr(index) }
    }

}


impl Drop for RawSystemChunk {
    fn drop(&mut self){
        free(self.data)
    }
}
