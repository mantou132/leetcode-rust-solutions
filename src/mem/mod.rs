#[cfg(debug_assertions)]
mod internal {
    extern "C" {
        fn __rust_allocate(size: usize, align: usize) -> *mut u8;
        fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize);
        fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8;
    }

    pub unsafe fn allocate(size: usize) -> *mut u8 {
        __rust_allocate(size, 0)
    }

    pub unsafe fn deallocate(ptr: *mut u8, old_size: usize) {
        __rust_deallocate(ptr, old_size, 0)
    }

    pub unsafe fn reallocate(ptr: *mut u8, old_size: usize, size: usize) -> *mut u8 {
        __rust_reallocate(ptr, old_size, size, 0)
    }
}

#[cfg(not(debug_assertions))]
mod internal {
    extern "C" {
        fn malloc(size: usize) -> *mut u8;
        fn free(ptr: *mut u8);
        fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    }

    #[inline(always)]
    pub unsafe fn allocate(size: usize) -> *mut u8 {
        malloc(size)
    }

    #[inline(always)]
    pub unsafe fn deallocate(ptr: *mut u8, _old_size: usize) {
        free(ptr)
    }

    #[inline(always)]
    pub unsafe fn reallocate(ptr: *mut u8, _old_size: usize, size: usize) -> *mut u8 {
        realloc(ptr, size)
    }
}

pub use self::internal::{allocate,deallocate,reallocate};

pub mod array;
