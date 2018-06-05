#![feature(rustc_private)]
#![feature(ascii_ctype)]
#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

extern crate proc_macro;
extern crate syntax;
extern crate syntax_pos;

mod common;
mod scanf;
mod printf;

use proc_macro::{TokenStream, Span};

#[proc_macro]
pub fn scanf(stream: TokenStream) -> TokenStream {
    scanf::parse_scanf(Span::call_site(), &mut stream.into_iter())
}

#[proc_macro]
pub fn printf(stream: TokenStream) -> TokenStream {
    printf::parse_printf(Span::call_site(), &mut stream.into_iter())
}
