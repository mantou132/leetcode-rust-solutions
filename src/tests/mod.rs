extern "C" {
    pub fn printf(fmt: *const u8, ...) -> i32;
    pub fn scanf(fmt: *const u8, ...) -> i32;
}


#[macro_export]
macro_rules! scanf {
    ($fmt:expr) => ({
        unsafe {
            $crate::tests::scanf(concat!($fmt, "\0").as_ptr());
        }
    });

    ($fmt:expr, $($arg:expr), +) => ({
        unsafe {
            $crate::tests::scanf(concat!($fmt, "\0").as_ptr(), $($arg), +);
        }
    });
}


#[macro_export]
macro_rules! printf {
    ($fmt:expr) => ({
        unsafe {
            $crate::tests::printf(concat!($fmt, "\0").as_ptr());
        }
    });

    ($fmt:expr, $($arg:expr), +) => ({
        unsafe {
            $crate::tests::printf(concat!($fmt, "\0").as_ptr(), $($arg), +);
        }
    });
}


mod io;
pub use self::io::*;
