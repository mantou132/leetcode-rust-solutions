use super::pool::{Handle as PoolHandle, Pool};
use core::fmt::Debug;
use core::mem::size_of;
use core::ptr::{null_mut, read, write};

pub trait Allocator {
    type Error: Debug;

    fn reallocate(&mut self, ptr: *mut u8, capacity: usize) -> Result<*mut u8, Self::Error>;
}

pub fn reallocate<T, A: Allocator>(allocator: &mut A, ptr: *mut T, mut capacity: isize) -> *mut T {
    if capacity < 0 {
        capacity = 0
    }
    let size = size_of::<T>();
    Allocator::reallocate(allocator, ptr as *mut _, size * (capacity as usize)).unwrap() as *mut _
}

pub fn allocate<T, A: Allocator>(allocator: &mut A, capacity: isize) -> *mut T {
    reallocate(allocator, null_mut(), capacity)
}

pub fn deallocate<T, A: Allocator>(allocator: &mut A, ptr: *mut T) {
    reallocate(allocator, ptr, 0);
}

#[derive(Copy, Clone, Eq)]
pub struct Handle(*mut u8);

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

impl PoolHandle for Handle {}

impl<T, A: Allocator> Pool<T> for A {
    type Handle = Handle;

    fn get(&self, handle: Self::Handle) -> &T {
        unsafe { &*(handle.0 as *mut T) }
    }

    fn get_mut(&mut self, handle: Self::Handle) -> &mut T {
        unsafe { &mut *(handle.0 as *mut T) }
    }

    fn add(&mut self, item: T) -> Self::Handle {
        unsafe {
            let ptr = allocate(self, 1);
            write(ptr as *mut T, item);
            Handle(ptr as *mut _)
        }
    }

    fn remove(&mut self, handle: Self::Handle) -> T {
        unsafe {
            let item = read(handle.0 as *mut T);
            deallocate(self, handle.0 as *mut T);
            item
        }
    }
}
