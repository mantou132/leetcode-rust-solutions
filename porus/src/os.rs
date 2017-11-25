use super::compat::prelude::*;
use std::fmt;
use std::ptr::null_mut;

mod sys {
    extern "C" {
        pub fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
        pub fn write(fd: i32, buf: *const u8, count: usize) -> isize;

        pub fn malloc(size: usize) -> *mut u8;
        pub fn free(ptr: *mut u8);
        pub fn realloc(ptr: *mut u8, size: usize) -> *mut u8;

        #[cfg_attr(target_os="windows", link_name = "_errno")]
        #[cfg_attr(target_os="linux", link_name = "__errno_location")]
        fn errno_location() -> *mut i32;
    }

    pub fn get_errno() -> i32 {
        unsafe {
            *errno_location()
        }
    }
}


#[derive(Debug)]
pub struct OSError(i32);

impl fmt::Display for OSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let OSError(n) = *self;
        write!(f, "OSError({})", n)
    }
}

fn get_error<T>() -> Result<T, OSError> {
    Err(OSError(sys::get_errno()))
}

pub fn read(fd: i32, buf: *mut u8, count: usize) -> Result<usize, OSError> {
    let mut length = 0;
    let mut ptr = buf;

    while length < count {
        let size =
            unsafe {
                sys::read(fd, ptr, count-length)
            };

        if size < 0 {
            return get_error();
        }
        if size == 0 {
            break;
        }

        length += size as usize;

        unsafe {
            ptr = ptr.offset(size);
        }
    }

    Ok(length)
}


pub fn write(fd: i32, buf: *const u8, count: usize) -> Result<(), OSError> {
    let mut written = 0;
    let mut ptr = buf;
    while written < count {
        let size =
            unsafe {
                sys::write(fd, ptr, count-written)
            };

        if size < 0 {
            return get_error();
        }

        written += size as usize;

        unsafe {
            ptr = ptr.offset(size);
        }
    }

    Ok(())
}


pub fn malloc(size: usize) -> Result<*mut u8, OSError> {
    if size == 0 {
        return Ok(null_mut());
    }

    let ptr = unsafe { sys::malloc(size) };
    if ptr.is_null() {
        return get_error();
    }

    Ok(ptr)
}

pub fn free(ptr: *mut u8) {
    if !(ptr.is_null()) {
        unsafe {
            sys::free(ptr)
        }
    }
}

pub fn realloc(ptr: *mut u8, size: usize) -> Result<*mut u8, OSError> {
    if ptr.is_null() {
        return malloc(size);
    }

    if size == 0 {
        free(ptr);
        return Ok(null_mut());
    }

    let p = unsafe { sys::realloc(ptr, size) };

    if p.is_null() {
        return get_error();
    }

    Ok(p)
}
