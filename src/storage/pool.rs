use core::ptr::{null_mut, read, write};
use core::mem::size_of;
use core::cmp::max;
use super::super::libc::{malloc, free};
use super::super::traits::{Allocator, Bounded};


pub struct Pool<T> {
    capacity: usize,
    data: *mut T,
    next: usize,
    empty: *mut T,
}


impl<T> Pool<T> {

    pub fn new_with_capacity(capacity: usize) -> Self {
        let ptr =
            unsafe {
                malloc(max(size_of::<T>(),size_of::<usize>()) * capacity) as *mut T
            };

        Pool {
            capacity: capacity,
            data: ptr,
            next: 0,
            empty: null_mut(),
        }
    }

    fn get_ptr_mut(&self, index : usize) -> *mut T {
        if size_of::<T>() < size_of::<usize>() {
            unsafe {
                (self.data as *mut usize).offset(index as isize) as *mut T
            }
        } else {
            unsafe {
                self.data.offset(index as isize)
            }
        }
    }
}

impl<T> Bounded for Pool<T> {
    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Allocator for Pool<T> {
    type Item = T;

    fn add(&mut self, item: T) -> *mut T {
        if self.empty != null_mut() {
            let ptr = self.empty;
            unsafe {
                self.empty = *(ptr as *mut *mut T);
                write(ptr, item);
            }
            ptr
        } else if self.next < self.capacity {
            let ptr = self.get_ptr_mut(self.next);
            self.next += 1;
            unsafe {
                write(ptr, item);
            }
            ptr
        } else {
            abort!("overflow");
        }
    }

    fn remove(&mut self, ptr: *mut T) -> T {
        if (ptr < self.get_ptr_mut(0)) || (ptr >= self.get_ptr_mut(self.capacity)) {
            #[cfg(debug_assertions)]
            abort!("pointer out of range");
        }

        unsafe {
            let item = read(ptr);
            let p = ptr as *mut *mut T;
            *p = self.empty;
            self.empty = ptr;
            item
        }
    }
}


impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.data as *mut u8);
        }
    }
}
