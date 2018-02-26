extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;


#[proc_macro_derive(List)]
pub fn derive_list(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Index<isize> for #name #ty_generics #where_clause {
            type Output = <Self as ListBase>::Element;

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
        impl #impl_generics IndexMut<isize> for #name #ty_generics #where_clause {
            fn index_mut(&mut self, index: isize) -> &mut Self::Output {
                ListMutBase::get_mut(self, index).unwrap()
            }
        }

        impl #impl_generics ListMut for #name #ty_generics #where_clause {
        }
    };

    expanded.into()
}
