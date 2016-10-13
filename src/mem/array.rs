use core::mem::size_of;
use core::ptr;

use super::{malloc, realloc, free};


pub struct Array<T> {
    capacity: usize,
    data: *mut T,
}

impl<T> Array<T> {
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn as_ptr(&mut self) -> *const T {
        self.data
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }

    pub fn new(capacity: usize) -> Array<T> {
        let size = size_of::<T>() * capacity;
        Array {
            capacity: capacity,
            data: unsafe { malloc(size) as *mut _ },
        }
    }

    pub fn resize(&mut self, capacity: usize) {
        let size = size_of::<T>() * capacity;
        self.data = unsafe{
            realloc(self.data as *mut _, size) as *mut _
        };

        self.capacity = capacity;
    }

    pub fn write(&mut self, index: usize, item: T){
        if index >= self.capacity {
            abort!("index out of range");
        }

        unsafe {
            ptr::write(self.data.offset(index as isize), item);
        }
    }

    pub fn read(&mut self, index: usize) -> T {
        if index >= self.capacity {
            abort!("index out of range");
        }

        unsafe {
            ptr::read(self.data.offset(index as isize))
        }
    }

    pub fn get<'a>(&'a self, index: usize) -> &'a T {
        if index >= self.capacity {
            abort!("index out of range");
        }

        unsafe {
            &*self.data.offset(index as isize)
        }
    }

}

impl<T> Drop for Array<T>{
    fn drop(&mut self){
        unsafe{
            free(self.data as *mut _)
        }
    }
}
