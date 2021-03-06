use common::parse_args;
use fmt_macros::{Argument, Count, Parser, Piece, Position};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::ToTokens;
use syn::{Expr, LitStr};

pub fn f(tokens: TokenStream) -> TokenStream {
    let (s, args): (LitStr, Expr) = parse_args(tokens).unwrap();

    let mut stream = quote!{};
    for p in Parser::new(s.value().as_str(), None) {
        match p {
            Piece::String(s) => {
                let lit = Literal::string(s);
                stream = quote! { #stream porus::io::write::fwrite_str(porus_sink, #lit); };
            }
            Piece::NextArgument(Argument {
                position: pos,
                format: fmt,
            }) => {
                let arg: Box<ToTokens> = match pos {
                    Position::ArgumentNamed(name) => Box::new(Ident::new(name, Span::call_site())),
                    Position::ArgumentImplicitlyIs(i) => {
                        let lit = Literal::usize_unsuffixed(i);
                        Box::new(quote! { porus_args.#lit })
                    }
                    Position::ArgumentIs(i) => {
                        let lit = Literal::usize_unsuffixed(i);
                        Box::new(quote! { porus_args.#lit })
                    }
                };

                match fmt.ty {
                    "" => {
                        stream =
                            quote! { #stream porus::io::write::fwrite(porus_sink, &mut #arg); };
                    }
                    "c" => {
                        stream = quote! { #stream porus::io::Sink::write(porus_sink, #arg); };
                    }
                    "s" => {
                        stream =
                            quote! { #stream porus::io::write::String::write(#arg, porus_sink); };
                    }
                    "d" => {
                        stream = quote! { #stream porus::io::write::Int::write(#arg, porus_sink, 10, 1); };
                    }
                    "f" => {
                        let prec: Box<ToTokens> = match fmt.precision {
                            Count::CountIs(n) => Box::new(Literal::i32_suffixed(n as _)),
                            Count::CountIsName(name) => {
                                Box::new(Ident::new(name, Span::call_site()))
                            }
                            Count::CountIsParam(i) => {
                                let lit = Literal::usize_unsuffixed(i);
                                Box::new(quote! { porus_args.#lit })
                            }
                            Count::CountImplied => {
                                panic!("precision is required by floating number format")
                            }
                        };

                        stream = quote! { #stream porus::io::write::Float::write(#arg, porus_sink, #prec); };
                    }
                    x => {
                        panic!("unknown format: {}", x);
                    }
                }
            }
        }
    }

    quote!{
        {
            #[allow(unused_variables, unused_mut)]
            let mut porus_args = #args;
            #[allow(unused_variables)]
            move |porus_sink : &mut _| {
                #stream
            }
        }
    }
}

pub fn writef(tokens: TokenStream) -> TokenStream {
    let stream = f(tokens);
    quote! {
        ::io::write(&mut #stream);
    }
}

pub fn writelnf(tokens: TokenStream) -> TokenStream {
    let stream = f(tokens);
    quote! {
        ::io::writeln(&mut #stream);
    }
}
