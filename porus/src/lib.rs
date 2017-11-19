// #![feature(specialization)]
#![feature(core_intrinsics)]
#![feature(try_trait)]
#![feature(proc_macro)]

extern crate porus_macros;

#[macro_use]
pub mod abort;

pub mod compat;
pub mod prelude;

pub mod os;

pub mod ctype;
pub mod storage;
#[macro_use]
pub mod io;

// #[macro_use]
// pub mod string;
// pub mod collections;
