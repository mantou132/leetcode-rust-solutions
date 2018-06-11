pub use core::cmp::Ordering::Less;
pub use core::cmp::Ordering::Equal;
pub use core::cmp::Ordering::Greater;

pub use super::iter::{Iter, IterRef, IterRefMut, into_iter};

pub use super::collection;
pub use super::list;
pub use super::list::slice;
pub use super::stack::Stack;
pub use super::deque::Deque;

pub use super::dlist::DoublyLinkedList;

pub use super::io;
pub use porus_macros::scanf;
pub use porus_macros::printf;

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
