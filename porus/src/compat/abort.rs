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
    () => ({ unsafe { ::core::intrinsics::abort(); }});
    ($msg:expr) => ({ unsafe { ::core::intrinsics::abort(); }});
    ($msg:expr, $($arg:tt)+) => ({ unsafe { ::core::intrinsics::abort(); }});
}

#[cfg(not(any(test, debug_assertions)))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[cfg(not(any(test, debug_assertions)))]
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    unsafe { ::core::intrinsics::abort() }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected="message")]
    fn test_abort() {
        abort!("message");
    }
}
