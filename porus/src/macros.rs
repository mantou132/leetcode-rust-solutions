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
