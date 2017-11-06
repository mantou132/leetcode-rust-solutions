// #![feature(specialization)]
#![feature(core_intrinsics)]
#![feature(try_trait)]

#[macro_use]
pub mod abort;

pub mod compat;
pub mod prelude;
#[macro_use]
pub mod macros;

pub mod os;

pub mod ctype;
pub mod storage;
pub mod io;

// #[macro_use]
// pub mod string;
// pub mod collections;
