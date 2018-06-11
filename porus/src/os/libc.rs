use super::OSError;

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

fn get_errno() -> i32 {
    unsafe {
        *errno_location()
    }
}

pub fn get_error<T>() -> Result<T, OSError> {
    Err(OSError(get_errno()))
}
