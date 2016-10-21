extern "C" {
    pub fn abort() -> !;
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! abort {
    ($msg:expr) => ({
        panic!($msg);
    });
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! abort {
    ($msg:expr) => ({
        $crate::libc::write_abort_msg(file!(), line!(), $msg);
        unsafe {
            $crate::libc::abort();
        }
    });
}

mod internal {
    extern "C" {
        pub fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
        pub fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    }
}

pub fn read(fd: i32, buf: *mut u8, count: usize) -> usize {
    let mut length = 0;
    let mut ptr = buf;

    while length < count {
        let size =
            unsafe {
                self::internal::read(fd, ptr, count-length)
            };

        if size < 0 {
            abort!("read error");
        }
        if size == 0 {
            break;
        }

        length += size as usize;

        unsafe {
            ptr = ptr.offset(size);
        }
    }

    length
}

pub fn write(fd: i32, buf: *const u8, count: usize) {
    let mut written = 0;
    let mut ptr = buf;
    while written < count {
        let size =
            unsafe {
                self::internal::write(fd, ptr, count-written)
            };

        if size < 0 {
            unsafe {
                abort();
            }
        }

        written += size as usize;

        unsafe {
            ptr = ptr.offset(size);
        }
    }
}

fn write_u32_aux(fd: i32, n: u32) {
    if n > 0 {
        write_u32_aux(fd, n / 10);
        let c = b'0' + ((n % 10) as u8);
        write(fd, &c, 1);
    }
}

fn write_u32(fd: i32, n: u32) {
    if n == 0 {
        write(fd, b"0".as_ptr(), 1);
    } else {
        write_u32_aux(fd, n);
    }
}

pub fn write_abort_msg(file: &str, line: u32, msg: &str) {
    // "panicked at '%.*s', %.*s:%u\n"
    write(2, b"panicked at '".as_ptr(), 13);
    write(2, msg.as_ptr(), msg.len());
    write(2, b"', ".as_ptr(), 3);
    write(2, file.as_ptr(), file.len());
    write(2, b":".as_ptr(), 1);
    write_u32(2, line);
    write(2, b"\n".as_ptr(), 1);
}


extern "C" {
    pub fn malloc(size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);
    pub fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
}


#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe {
        malloc(size)
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe {
        free(ptr)
    }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                _align: usize) -> *mut u8 {
    unsafe {
        realloc(ptr, size)
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}


extern "C" {
    pub fn printf(fmt: *const u8, ...) -> i32;
    pub fn scanf(fmt: *const u8, ...) -> i32;
}


#[macro_export]
macro_rules! scanf {
    ($fmt:expr) => ({
        unsafe {
            $crate::libc::scanf(concat!($fmt, "\0").as_ptr());
        }
    });

    ($fmt:expr, $($arg:expr), +) => ({
        unsafe {
            $crate::libc::scanf(concat!($fmt, "\0").as_ptr(), $($arg), +);
        }
    });
}


#[macro_export]
macro_rules! printf {
    ($fmt:expr) => ({
        unsafe {
            $crate::libc::printf(concat!($fmt, "\0").as_ptr());
        }
    });

    ($fmt:expr, $($arg:expr), +) => ({
        unsafe {
            $crate::libc::printf(concat!($fmt, "\0").as_ptr(), $($arg), +);
        }
    });
}
