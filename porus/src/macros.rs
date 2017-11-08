#[macro_export]
macro_rules! main {
    ($name:ident) => (
        #[cfg_attr(not(debug_assertions), no_mangle)]
        pub fn main() {
            $name().unwrap()
        }
    )
}

#[macro_export]
macro_rules! prelude {
    ($name:ident) => (
        use $crate::prelude::*;
        main!($name);
    )
}
