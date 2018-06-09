use std::mem::size_of;
use std::ptr::{read, write, null_mut};
use super::libc::{malloc, free};
use super::pool::{Pool, Handle as PoolHandle};


pub struct OSAllocator {
}

#[derive(Copy, Clone, Eq)]
pub struct Handle (*mut u8);

impl PartialEq for Handle {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl Default for Handle {
    fn default() -> Self {
        Handle(null_mut())
    }
}

impl PoolHandle for Handle {
}

impl<T> Pool<T> for OSAllocator {
    type Handle = Handle;

    fn get(&self, handle: Self::Handle) -> &T {
        unsafe {
            &*(handle.0 as *mut T)
        }
    }

    fn get_mut(&mut self, handle: Self::Handle) -> &mut T {
        unsafe {
            &mut *(handle.0 as *mut T)
        }
    }

    fn add(&mut self, item: T) -> Self::Handle {
        let size = size_of::<T>();
        unsafe {
            let ptr = malloc(size).unwrap();
            write(ptr as *mut T, item);
            Handle(ptr)
        }
    }

    fn remove(&mut self, handle: Self::Handle) -> T {
        unsafe {
            let item = read(handle.0 as *mut T);
            free(handle.0);
            item
        }
    }
}

impl OSAllocator {
    pub fn new() -> Self {
        OSAllocator {
        }
    }
}
