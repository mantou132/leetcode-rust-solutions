//! Compatibility layer to make same code can be compiled to normal
//! Rust binary locally, and in the same time can be submit to online
//! judge.
//!
//! Even with `panic="abort"`, the generated assembly file is usually
//! too big to submit. For example, `Option::unwrap` would `panic!` on
//! a `None` value. To completely avoid `panic!`, we have to define
//! our own version of `Option` here.

#[macro_use]
pub mod abort;
pub mod option;
pub mod result;
pub mod prelude;
