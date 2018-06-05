#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

mod common;
mod scanf;
mod printf;

use proc_macro::TokenStream;

#[proc_macro]
pub fn scanf(stream: TokenStream) -> TokenStream {
    scanf::parse_scanf(stream).unwrap()
}

#[proc_macro]
pub fn printf(stream: TokenStream) -> TokenStream {
    printf::parse_printf(stream).unwrap()
}
