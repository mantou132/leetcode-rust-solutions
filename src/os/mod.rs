use core::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct OSError(i32);

impl Display for OSError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let OSError(n) = *self;
        write!(f, "OSError({})", n)
    }
}

mod libc;

mod alloc;
pub use self::alloc::OSAllocator;

pub mod file;
