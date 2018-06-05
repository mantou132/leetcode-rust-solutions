// #![feature(specialization)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
#![feature(try_trait)]
#![feature(decl_macro)]
#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus_macros;

#[macro_use]
pub mod abort;

pub mod compat;
pub mod prelude;
#[macro_use]
pub mod macros;

pub mod ctype;
pub mod os;
pub mod chunk;
pub mod io;

#[macro_use]
pub mod iter;

#[macro_use]
pub mod range;

pub mod collection;
pub mod capacity;
pub mod list;
pub mod stack;

#[macro_use]
pub mod array;

// #[macro_use]
// pub mod string;
// pub mod collections;
