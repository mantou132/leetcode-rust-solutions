use core::mem::size_of;
use core::ptr;

use super::super::libc::{malloc, realloc, free};


pub struct Chunk<T> {
    capacity: usize,
    data: *mut T,
}

impl<T> Chunk<T> {

    pub fn with_capacity(capacity: usize) -> Self {
        let size = size_of::<T>() * capacity;
        Chunk {
            capacity: capacity,
            data: unsafe { malloc(size) as *mut _ },
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn as_ptr(&mut self) -> *const T {
        self.data
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }

    pub fn resize(&mut self, capacity: usize) {
        let size = size_of::<T>() * capacity;
        self.data = unsafe{
            realloc(self.data as *mut _, size) as *mut _
        };

        self.capacity = capacity;
    }

    pub fn get_ptr(&self, index: usize) -> *const T {
        if index >= self.capacity {
            #[cfg(debug_assertions)]
            abort!("index out of range");
        }

        unsafe {
            self.data.offset(index as isize)
        }
    }

    pub fn get_ptr_mut(&self, index: usize) -> *mut T {
        if index >= self.capacity {
            #[cfg(debug_assertions)]
            abort!("index out of range");
        }

        unsafe {
            self.data.offset(index as isize)
        }
    }

    pub fn write(&mut self, index: usize, item: T){
        unsafe {
            ptr::write(self.get_ptr_mut(index), item);
        }
    }

    pub fn read(&mut self, index: usize) -> T {
        unsafe {
            ptr::read(self.get_ptr(index))
        }
    }

    pub fn get(&self, index: usize) -> &T {
        unsafe {
            &*self.get_ptr(index)
        }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        unsafe {
            &mut *self.get_ptr_mut(index)
        }
    }

}

impl<T> Drop for Chunk<T>{
    fn drop(&mut self){
        unsafe{
            free(self.data as *mut _)
        }
    }
}
