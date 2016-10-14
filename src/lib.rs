#![feature(allocator)]
#![allocator]
#![no_std]

#[macro_use]
pub mod libc;
pub mod ctype;
pub mod traits;
pub mod mem;
pub mod iter;
pub mod string;
pub mod io;

#[cfg(debug_assertions)]
#[macro_use]
pub mod tests;
