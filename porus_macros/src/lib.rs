#![feature(rustc_private)]
#![feature(ascii_ctype)]
#![feature(proc_macro)]

extern crate proc_macro;
extern crate syntax;
extern crate syntax_pos;

mod common;
mod scanf;
mod printf;

use proc_macro::{TokenStream, Span};

#[proc_macro]
pub fn scanf_impl(stream: TokenStream) -> TokenStream {
    match scanf::parse_scanf(Span::call_site(), &mut stream.into_iter()) {
        Ok(s) => s,
        Err(()) => TokenStream::empty(),
    }
}

#[proc_macro]
pub fn printf_impl(stream: TokenStream) -> TokenStream {
    match printf::parse_printf(Span::call_site(), &mut stream.into_iter()) {
        Ok(s) => s,
        Err(()) => TokenStream::empty(),
    }
}
