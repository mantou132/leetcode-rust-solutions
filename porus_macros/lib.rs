#![feature(rustc_private)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate fmt_macros;

use proc_macro::{Group, Span, TokenStream, TokenTree};
use std::iter::FromIterator;

mod common;
mod format;

fn set_span(span: Span, stream: TokenStream) -> TokenStream {
    let iter = stream.into_iter().map(|mut tree| match tree {
        TokenTree::Group(g) => {
            TokenTree::Group(Group::new(g.delimiter(), set_span(span, g.stream())))
        }
        _ => {
            tree.set_span(span);
            tree
        }
    });
    TokenStream::from_iter(iter)
}

#[proc_macro]
pub fn f(stream: TokenStream) -> TokenStream {
    set_span(Span::call_site(), format::f(stream.into()).into())
}

#[proc_macro]
pub fn writef(stream: TokenStream) -> TokenStream {
    set_span(Span::call_site(), format::writef(stream.into()).into())
}

#[proc_macro]
pub fn writelnf(stream: TokenStream) -> TokenStream {
    set_span(Span::call_site(), format::writelnf(stream.into()).into())
}

use syn::DeriveInput;

#[proc_macro_derive(List)]
pub fn derive_list(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::core::ops::Index<isize> for #name #ty_generics #where_clause {
            type Output = <Self as ListBase>::Elem;

            fn index(&self, index: isize) -> &Self::Output {
                ListBase::get(self, index).unwrap()
            }
        }

        impl #impl_generics List for #name #ty_generics #where_clause {
        }
    };

    expanded.into()
}

#[proc_macro_derive(ListMut)]
pub fn derive_listmut(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::core::ops::IndexMut<isize> for #name #ty_generics #where_clause {
            fn index_mut(&mut self, index: isize) -> &mut Self::Output {
                ListMutBase::get_mut(self, index).unwrap()
            }
        }

        impl #impl_generics ListMut for #name #ty_generics #where_clause {
        }
    };

    expanded.into()
}
