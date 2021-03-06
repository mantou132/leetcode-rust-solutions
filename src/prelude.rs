pub use core::cmp::Ordering::Equal;
pub use core::cmp::Ordering::Greater;
pub use core::cmp::Ordering::Less;

pub use core::f64::consts::PI;
use core::intrinsics::sqrtf64;

pub fn sqrt(x: f64) -> f64 {
    unsafe { sqrtf64(x) }
}

pub use super::iter::{into_iter, IterRef, IterRefMut, Iterator};

pub use super::collection;
pub use super::deque::Deque;
pub use super::list;
pub use super::list::slice;
pub use super::stack;

pub use super::array::Array;
pub use super::dlist::DoublyLinkedList;

pub use super::io::read::Char;
pub use super::io::write::join;
pub use porus_macros::{f, writef, writelnf};

pub fn default<T: Default>() -> T {
    Default::default()
}

#[macro_export]
macro_rules! read_opt {
    () => {{
        let mut x = Default::default();
        if ::io::read_skip_ws(&mut x) {
            Some(x)
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! read {
    () => (
        {
            read_opt!().unwrap()
        }
    );
    ( $($expr:expr),* ) => (
        $(
            ::io::read_skip_ws($expr);
        )*
    )
}

/// the porus prelude
#[macro_export]
macro_rules! prelude {
    () => {
        prelude!(1024);
    };
    ($size:expr) => {
        #[allow(unused_imports)]
        use $crate::prelude::*;

        mod io {
            #[cfg(debug_assertions)]
            use std::ptr::drop_in_place;

            #[cfg(not(debug_assertions))]
            use core::ptr::drop_in_place;

            use $crate::io::read::{fread, Consumer, Whitespace};
            use $crate::io::stdio;
            use $crate::io::write::fwrite;
            use $crate::io::Sink;

            #[allow(dead_code)]
            static mut STDIN: stdio::Input = stdio::stdin(&mut [0; $size]);
            static mut STDOUT: stdio::Output = stdio::stdout(&mut [0; $size]);

            #[allow(dead_code)]
            pub fn read<C: Consumer>(c: C) -> bool {
                unsafe { fread(&mut STDIN, c) }
            }

            pub fn read_skip_ws<C: Consumer>(c: C) -> bool {
                read(Whitespace);
                read(c)
            }

            #[allow(dead_code)]
            pub fn write<'a, F: FnMut(&'a mut stdio::Output)>(f: &mut F) {
                unsafe {
                    fwrite(&mut STDOUT, f);
                }
            }

            #[allow(dead_code)]
            pub fn writeln<'a, F: FnMut(&'a mut stdio::Output)>(f: &mut F) {
                write(f);
                unsafe {
                    Sink::write(&mut STDOUT, b'\n');
                }
            }

            pub fn main() {
                ::solve();
                unsafe { drop_in_place(&mut STDOUT as *mut _) };
            }
        }

        #[cfg(debug_assertions)]
        fn main() {
            io::main();
        }

        #[cfg(not(debug_assertions))]
        #[no_mangle]
        pub extern "C" fn main() -> i32 {
            io::main();
            0
        }
    };
}
