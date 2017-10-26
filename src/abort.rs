use super::compat::prelude::*;
use super::os::{write, OSError};


#[cfg(debug_assertions)]
#[macro_export]
macro_rules! abort {
    () => ({ panic!() });
    ($msg:expr) => ({ panic!($msg); });
    ($fmt:expr, $($arg:tt)+) => { panic!($fmt, $($arg)+) };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! abort {
    () => ({ unsafe { ::std::intrinsics::abort(); }});
    ($msg:expr) => ({ unsafe { ::std::intrinsics::abort(); }});
    ($msg:expr, $($arg:tt)+) => ({ unsafe { ::std::intrinsics::abort(); }});
}


fn write_u32_aux(fd: i32, n: u32) -> Result<(),OSError> {
    if n > 0 {
        write_u32_aux(fd, n / 10)?;
        let c = b'0' + ((n % 10) as u8);
        write(fd, &c, 1)?;
    }
    Ok(())
}

fn write_u32(fd: i32, n: u32) -> Result<(),OSError> {
    if n == 0 {
        write(fd, "0".as_ptr(), 1)?;
    } else {
        write_u32_aux(fd, n)?;
    }
    Ok(())
}

pub fn write_abort_msg(file: &str, line: u32, msg: &str) -> Result<(),OSError> {
    // "panicked at '%.*s', %.*s:%u\n"
    write(2, "panicked at '".as_ptr(), 13)?;
    write(2, msg.as_ptr(), msg.len())?;
    write(2, "', ".as_ptr(), 3)?;
    write(2, file.as_ptr(), file.len())?;
    write(2, ":".as_ptr(), 1)?;
    write_u32(2, line)?;
    write(2, "\n".as_ptr(), 1)?;
    Ok(())
}

