#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
use porus::prelude::*;

#[no_mangle]
pub extern fn divide(dividend: i32, divisor: i32) -> i32 {
    match dividend.overflowing_div(divisor) {
        (x, false) => x,
        (_, true) => i32::max_value(),
    }
}
