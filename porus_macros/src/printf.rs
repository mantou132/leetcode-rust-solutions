use proc_macro::TokenStream;
use proc_macro2::Literal;
use syn::buffer::TokenBuffer;
use syn::synom::ParseError;
use syn::Expr;
use syn::token::Comma;
use syn::LitStr;
use std::str::Chars;
use std::iter::Peekable;
use common::Cursor;

#[derive(Debug)]
enum Directive {
    Literal(char),
    Char,
    Int(u8),
    String,
}

fn parse_printf_conversion(fmt: &mut Peekable<Chars>) -> Directive {
    match fmt.next() {
        None => {
            panic!("conversion lacks type at end of format");
        },
        Some('c') => Directive::Char,
        Some('d') => Directive::Int(10),
        Some('s') => Directive::String,
        Some(c) => {
            panic!("unknown conversion type character `{}`", c);
        },
    }
}

fn parse_printf_fmt(fmt: &mut Peekable<Chars>) -> Vec<Directive> {
    let mut dirs = Vec::new();

    while let Some(&c) = fmt.peek() {
        fmt.next();

        let d =
            if c == '%' {
                if let Some(&'%') = fmt.peek() {
                    fmt.next();
                    Directive::Literal('%')
                } else {
                    parse_printf_conversion(fmt)
                }
            } else {
                Directive::Literal(c)
            };

        dirs.push(d);
    }

    dirs
}

pub fn parse_printf(tokens: TokenStream) -> Result<TokenStream, ParseError> {
    let buf = TokenBuffer::new2(tokens.into());
    let mut cur = Cursor::new(&buf);
    let file : Expr = cur.parse()?;
    let _ : Comma = cur.parse()?;
    let s : LitStr = cur.parse()?;
    let fmt = parse_printf_fmt(&mut s.value().chars().peekable());

    let mut stream = quote!{ #file };
    for d in fmt.into_iter() {
        match d {
            Directive::Literal(c) => {
                let c = Literal::u8_suffixed(c as _);
                stream = quote!{ io::printf::write_char(#stream, #c) };
            },
            Directive::Char => {
                let _ : Comma = cur.parse()?;
                let param : Expr = cur.parse()?;
                stream = quote!{ io::printf::write_char(#stream, #param) };
            },
            Directive::Int(x) => {
                let _ : Comma = cur.parse()?;
                let param : Expr = cur.parse()?;
                let base = Literal::u8_suffixed(x);
                stream = quote!{ io::printf::write_string(#stream, io::printf::IntField::converter(#param, #base)) };
            },
            Directive::String => {
                let _ : Comma = cur.parse()?;
                let param : Expr = cur.parse()?;
                stream = quote!{ io::printf::write_string(#stream, #param) };
            },
        }
    }

    Ok(stream.into())
}
