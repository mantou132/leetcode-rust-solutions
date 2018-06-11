#![feature(core_intrinsics)]
#![feature(try_trait)]
#![feature(decl_macro)]
#![feature(proc_macro_non_items)]
#![feature(extern_prelude)]
#![cfg_attr(not(any(test, debug_assertions)), feature(lang_items))]
#![cfg_attr(not(any(test, debug_assertions)), feature(panic_implementation))]
#![no_std]

//! [`porus`](self) is a library for competitive programming. It is at
//! a very early stage of development. USE AT YOUR OWN RISK.
//!

//! Competitive programming still stucks in the 1950s when you have to
//! build everything from scratch, since almost all popular online
//! judges accept only a single file within tens of kilobytes. So
//! before submitting, you have to generate a file from your solution.
//!

//! Currently Rust nightly is required, and you have to put following
//! code at the very beginning of your solutions.
//!
//! ```ignore
//! #![feature(proc_macro_non_items)]
//! #![cfg_attr(not(debug_assertions), no_std)]
//!
//! #[macro_use]
//! extern crate porus;
//! prelude!();
//! ```
//!

//! ## Abstract Data Types
//! * [`Pool`](pool)
//! * [`Allocator`](alloc)
//! * [`Collection`](collection)
//! * [`List`](list)
//! * [`Stack`](stack)
//! * [`Deque`](deque)
//!

//! ## Data Structures
//! * [`Array`](type@array) : [`List`](list) + [`Stack`](stack)
//! * [`Buffer`](type@buffer) : [`List`](list) + [`Deque`](deque)
//! * [`DoublyLinkedList`](type@dlist) : [`Deque`](deque)
//!

#[macro_use]
extern crate porus_macros;

#[macro_use]
pub mod compat;
pub mod ptr;

#[macro_use]
pub mod range;

pub mod capacity;
pub mod pool;
pub mod alloc;

#[macro_use]
pub mod iter;
pub mod collection;
pub mod list;
pub mod stack;
pub mod deque;

#[macro_use]
pub mod array;
#[macro_use]
pub mod buffer;
pub mod dlist;

pub mod io;
pub mod os;
#[macro_use]
pub mod prelude;
