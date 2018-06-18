use proc_macro::TokenStream;
use proc_macro2::{Span, Literal, Ident};
use syn::{Expr, LitStr};
use fmt_macros::{Parser, Piece, Argument, Position};
use common::parse_args;
use quote::ToTokens;

pub fn f(tokens: TokenStream) -> TokenStream {
    let (s, args) : (LitStr, Vec<Expr>) = parse_args(tokens).unwrap();

    let mut stream = quote!{ };
    for p in Parser::new(s.value().as_str()) {
        match p {
            Piece::String(s) => {
                let lit = Literal::string(s);
                stream = quote! { #stream porus::io::write::fwrite_str(sink, #lit); };
            },
            Piece::NextArgument(Argument{position: pos, format: fmt}) => {
                let arg : Box<ToTokens> =
                    match pos {
                        Position::ArgumentNamed(name) => { Box::new(Ident::new(name, Span::call_site())) },
                        Position::ArgumentImplicitlyIs(i) => { Box::new(args[i].clone()) },
                        Position::ArgumentIs(i) => { Box::new(args[i].clone()) },
                    };

                match fmt.ty {
                    "" => {
                        stream = quote! { #stream porus::io::write::String::write(#arg, sink); };
                    },
                    "d" => {
                        stream = quote! { #stream porus::io::write::Int::write(#arg, sink, 10); };
                    },
                    x => {
                        panic!("unknown format: {}", x);
                    }
                }

            }
        }
    };

    (quote!{
        move |sink| {
            #stream
        }
    }).into()
}
