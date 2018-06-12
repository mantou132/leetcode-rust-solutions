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

pub use core::ptr::drop_in_place;


#[macro_export]
macro_rules! prelude {
    () => (
        prelude!(1024);
    );
    ($size:expr) => (
        use $crate::prelude::*;

        #[allow(dead_code)]
        static mut STDIN : $crate::io::stdio::Input = $crate::io::stdin(&mut [0;$size]);
        static mut STDOUT : $crate::io::stdio::Output = $crate::io::stdout(&mut [0;$size]);

        fn porus_main() {
            solve();
            unsafe {
                drop_in_place(&mut STDOUT as *mut _)
            };
        }

        #[cfg(debug_assertions)]
        fn main() {
            porus_main();
        }

        #[cfg(not(debug_assertions))]
        #[no_mangle]
        pub extern fn main() -> i32 {
            porus_main();
            0
        }
    )
}
