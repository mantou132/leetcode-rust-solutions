use super::super::compat::prelude::*;
use std::ptr::null_mut;
use super::OSError;
use super::libc::{malloc, free, realloc, get_error};
use super::super::alloc::Allocator;

pub struct OSAllocator {
}

impl Default for OSAllocator {
    fn default() -> Self {
        OSAllocator {
        }
    }
}

impl Allocator for OSAllocator {
    type Error = OSError;

    fn reallocate(&mut self, ptr: *mut u8, size: usize) -> Result<*mut u8, OSError> {
        if size == 0 {
            if !ptr.is_null() {
                unsafe {
                    free(ptr);
                }
            }
            Ok(null_mut())
        } else {
            let p =
                if ptr.is_null() {
                    unsafe { malloc(size) }
                } else {
                    unsafe { realloc(ptr, size) }
                };

            if p.is_null() {
                get_error()
            } else {
                Ok(p)
            }
        }
    }
}

impl OSAllocator {
    pub fn new() -> Self {
        OSAllocator {
        }
    }
}
