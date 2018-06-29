#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(proc_macro_non_items)]
#![feature(extern_prelude)]
#![feature(const_fn)]
#![feature(const_slice_len)]
#![feature(const_slice_as_ptr)]
#![feature(try_from)]
#![feature(specialization)]
#![cfg_attr(not(any(test, debug_assertions)), feature(lang_items))]
#![cfg_attr(not(any(test, debug_assertions)), feature(panic_implementation))]
#![no_std]

//! [`porus`](self) is a library for competitive programming. It is at
//! a very early stage of development. USE AT YOUR OWN RISK.
//!

//! Competitive programming still stucks in the 1950s when you have to
//! build everything from scratch. Everyone is rolling their own
//! ad-hoc implementations of data structures and algorithms, since
//! most popular online judges accept only a single file within tens
//! of kilobytes which is orders of magnitude smaller than the size of
//! libraries used in real world.
//!

//! Thus, solutions have to be preproccessed before submitting to
//! online judges. Right now, [`porus`](self) piggybacks on `ix` to do
//! the preprocessing.  For example, to submit to
//! [AOJ](http://judge.u-aizu.ac.jp/onlinejudge/) the solution to
//! [ITP1_1_A](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A)
//!

//! ```bash
//! $ python3 -mix submit -w solutions/AOJ/ITP1/ITP1_1_A.rs
//! [SUBMIT] solutions/AOJ/ITP1/ITP1_1_A.rs
//! [COMPILE] target/x86_64-unknown-linux-gnu/release/libporus.rlib
//! AOJ (judge.u-aizu.ac.jp)
//! User ID: your_username
//! Password:
//! [SUBMIT] solutions/AOJ/ITP1/ITP1_1_A.rs: Accepted (Memory: 2000, Time: 0, Length: 1380)
//! $
//! ```
//!

//! Under the hood, Rust code is first compiled to assembly with
//! link-time optimization. Because of name mangling, labels in the
//! assembly code might consist of tens of characters. So, then all
//! labels is renamed, to reduce the size of the assembly
//! code. Finally, the assembly code is compressed with [Sequitur
//! algorithm](https://en.wikipedia.org/wiki/Sequitur_algorithm) to
//! furthur reduce the size, and wrapped with `__asm__()`, so that it
//! could be submitted as C code.
//!

//! Currently Rust nightly is required, and following code have to be
//! put at the very beginning of solution code.
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

pub mod ptr;

#[macro_use]
pub mod range;

pub mod capacity;
pub mod pool;
pub mod alloc;

#[macro_use]
pub mod iter;
pub mod collection;
#[macro_use]
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
