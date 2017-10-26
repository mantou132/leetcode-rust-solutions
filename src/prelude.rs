pub use super::compat::prelude::*;
pub use super::io;

#[cfg(debug_assertions)]
mod internal {
    use std::boxed::Box;

    pub type Error = Box<::std::error::Error>;
}


#[cfg(not(debug_assertions))]
mod internal {
    pub use super::super::compat::error::Error;
}

pub use self::internal::*;

pub fn default<T: Default>() -> T {
    Default::default()
}
