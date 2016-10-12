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
