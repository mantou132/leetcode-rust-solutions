#![feature(proc_macro)]
#![feature(proc_macro_non_items)]
#![feature(rustc_private)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate fmt_macros;

use proc_macro::{TokenStream, Span};

mod common;
use common::set_span;

mod format;

#[proc_macro]
pub fn f(stream: TokenStream) -> TokenStream {
    set_span(Span::call_site(), format::f(stream))
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
