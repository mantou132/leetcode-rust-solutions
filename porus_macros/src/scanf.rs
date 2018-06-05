use proc_macro::{TokenStream, Span};
use proc_macro2::Literal;
use syn::buffer::TokenBuffer;
use syn::synom::ParseError;
use syn::Expr;
use syn::token::Comma;
use syn::LitStr;
use std::str::Chars;
use std::iter::Peekable;
use common::{Cursor, set_span};

#[derive(Debug)]
enum Pattern {
    Char,
    Signed(u8),
    Unsigned(u8),
}

#[derive(Debug)]
enum Directive {
    Whitespace,
    Exact(char),
    Match(Pattern),
    Ignore(Pattern),
}

fn parse_scanf_pattern(fmt: &mut Peekable<Chars>) -> Pattern {
    match fmt.next() {
        None => {
            panic!("conversion lacks type at end of format");
        },
        Some('c') => Pattern::Char,
        Some('b') => Pattern::Unsigned(2),
        Some('o') => Pattern::Unsigned(8),
        Some('d') => Pattern::Signed(10),
        Some('x') => Pattern::Unsigned(16),
        Some(c) => {
            panic!("unknown conversion type character `{}`", c);
        },
    }
}

fn parse_scanf_fmt(fmt: &mut Peekable<Chars>) -> Vec<Directive> {
    let mut dirs = Vec::new();

    while let Some(&c) = fmt.peek() {
        fmt.next();

        let d =
            if c.is_ascii_whitespace() {
                while let Some(&c) = fmt.peek() {
                    if !(c.is_ascii_whitespace()) {
                        break;
                    }
                    fmt.next();
                }
                Directive::Whitespace
            } else if c == '%' {
                if let Some(&'%') = fmt.peek() {
                    fmt.next();
                    Directive::Exact('%')
                } else if let Some(&'*') = fmt.peek() {
                    fmt.next();
                    Directive::Ignore(parse_scanf_pattern(fmt))
                } else {
                    Directive::Match(parse_scanf_pattern(fmt))
                }
            } else {
                Directive::Exact(c)
            };

        dirs.push(d);
    }

    dirs
}


pub fn parse_scanf(tokens: TokenStream) -> Result<TokenStream, ParseError> {
    let buf = TokenBuffer::new2(tokens.into());
    let mut cur = Cursor::new(&buf);
    let file : Expr = cur.parse()?;
    let _ : Comma = cur.parse()?;
    let s : LitStr = cur.parse()?;
    let fmt = parse_scanf_fmt(&mut s.value().chars().peekable());

    let mut stream = quote!{ #file };

    for d in fmt.into_iter() {
        match d {
            Directive::Whitespace => {
                stream = quote!{ io::scanf::whitespace(#stream) };
            },
            Directive::Exact(c) => {
                let c = Literal::u8_suffixed(c as _);
                stream = quote!{ io::scanf::exact(#stream, #c) };
            },
            Directive::Ignore(Pattern::Char) => {
                stream = quote!{ io::scanf::character(#stream, &mut io::scanf::Ignore) };
            },
            Directive::Ignore(Pattern::Unsigned(x)) => {
                let base = Literal::u8_suffixed(x);
                stream = quote!{ io::scanf::unsigned(#stream, &mut io::scanf::Ignore, #base) };
            },
            Directive::Ignore(Pattern::Signed(x)) => {
                let base = Literal::u8_suffixed(x);
                stream = quote!{ io::scanf::signed(#stream, &mut io::scanf::Ignore, #base) };
            },
            Directive::Match(Pattern::Char) => {
                let _ : Comma = cur.parse()?;
                let param : Expr = cur.parse()?;
                stream = quote!{ io::scanf::character(#stream, &mut io::scanf::CharPattern::converter(#param)) };
            },
            Directive::Match(Pattern::Unsigned(x)) => {
                let _ : Comma = cur.parse()?;
                let param : Expr = cur.parse()?;
                let base = Literal::u8_suffixed(x);
                stream = quote!{ io::scanf::unsigned(#stream, &mut io::scanf::UnsignedPattern::converter(#param, #base), #base) };
            },
            Directive::Match(Pattern::Signed(x)) => {
                let _ : Comma = cur.parse()?;
                let param : Expr = cur.parse()?;
                let base = Literal::u8_suffixed(x);
                stream = quote!{ io::scanf::signed(#stream, &mut io::scanf::SignedPattern::converter(#param, #base), #base) };
            },
        }
    }

    Ok(set_span(Span::call_site(), stream.into()))
}
