//! replacement to `panic!`

#[cfg(any(test, debug_assertions))]
#[macro_export]
macro_rules! abort {
    () => ({ panic!() });
    ($msg:expr) => ({ panic!($msg); });
    ($fmt:expr, $($arg:tt)+) => { panic!($fmt, $($arg)+) };
}

#[cfg(not(any(test, debug_assertions)))]
#[macro_export]
macro_rules! abort {
    () => ({ unsafe { ::std::intrinsics::abort(); }});
    ($msg:expr) => ({ unsafe { ::std::intrinsics::abort(); }});
    ($msg:expr, $($arg:tt)+) => ({ unsafe { ::std::intrinsics::abort(); }});
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected="message")]
    fn test_abort() {
        abort!("message");
    }
}
