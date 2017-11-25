pub use super::compat::prelude::*;

#[allow(non_camel_case_types)]
pub type int = isize;

pub use std::cmp::Ordering::Less;
pub use std::cmp::Ordering::Equal;
pub use std::cmp::Ordering::Greater;

pub use super::io;

pub use super::list;
pub use super::list::{List, ListMut};

pub fn default<T: Default>() -> T {
    Default::default()
}
