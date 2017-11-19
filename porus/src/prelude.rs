pub use super::compat::prelude::*;

#[allow(non_camel_case_types)]
pub type int = isize;

pub use std::cmp::Ordering::Less;
pub use std::cmp::Ordering::Equal;
pub use std::cmp::Ordering::Greater;

pub use super::io;

pub fn default<T: Default>() -> T {
    Default::default()
}
