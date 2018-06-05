pub use super::compat::prelude::*;

pub use std::cmp::Ordering::Less;
pub use std::cmp::Ordering::Equal;
pub use std::cmp::Ordering::Greater;

pub use super::io;
pub use porus_macros::scanf;
pub use porus_macros::printf;

pub use super::iter::foreach;
pub use super::collection;
pub use super::list;
pub use super::list::slice;
pub use super::stack::Stack;
pub use super::deque::Deque;

pub fn default<T: Default>() -> T {
    Default::default()
}

#[macro_export]
macro_rules! prelude {
    () => (
        use $crate::prelude::*;

        #[cfg(debug_assertions)]
        fn main() {
            solve();
        }

        #[cfg(not(debug_assertions))]
        #[no_mangle]
        pub extern fn main() -> i32 {
            solve();
            0
        }
    )
}
