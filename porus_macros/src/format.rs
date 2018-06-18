use proc_macro::TokenStream;
use proc_macro2::{Span, Literal, Ident};
use syn;
use syn::LitStr;
use fmt_macros::{Parser, Piece, Argument, Position};


pub fn f(tokens: TokenStream) -> TokenStream {
    let s: LitStr = syn::parse(tokens).unwrap();
    let mut stream = quote!{ };
    for p in Parser::new(s.value().as_str()) {
        match p {
            Piece::String(s) => {
                let lit = Literal::string(s);
                stream = quote! { #stream porus::io::write::fwrite_str(sink, #lit); };
            },
            Piece::NextArgument(Argument{position: Position::ArgumentNamed(name), format: fmt}) => {
                let id = Ident::new(name, Span::call_site());
                match fmt.ty {
                    "" => {
                        stream = quote! { #stream porus::io::write::String::write(#id, sink); };
                    },
                    "d" => {
                        stream = quote! { #stream porus::io::write::Int::write(#id, sink, 10); };
                    },
                    x => {
                        panic!("unknown format: {}", x);
                    }
                }

            },
            _ => {
                panic!("positional parameter not allowed")
            }
        }
    };

    (quote!{
        move |sink| {
            #stream
        }
    }).into()
}
