use core::marker::PhantomData;
use core::mem::size_of;
use core::ptr::{read, write};
use super::super::libc::{malloc, free};
use super::super::traits::Allocator;

pub struct SystemAllocator<T> {
    _marker: PhantomData<T>,
}

impl<T> Allocator for SystemAllocator<T> {
    type Item = T;

    fn add(&mut self, item: T) -> *mut T {
        let size = size_of::<T>();
        unsafe {
            let ptr = malloc(size) as *mut T;
            write(ptr, item);
            ptr
        }
    }

    fn remove(&mut self, ptr: *mut T) -> T {
        unsafe {
            let item = read(ptr);
            free(ptr as *mut u8);
            item
        }
    }
}

impl<T> SystemAllocator<T> {
    pub fn new() -> Self {
        SystemAllocator {
            _marker: PhantomData
        }
    }
}
