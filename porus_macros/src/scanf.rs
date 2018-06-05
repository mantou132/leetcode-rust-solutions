use proc_macro::{quote, TokenStream, TokenTree, Span, Literal};
use proc_macro::token_stream::IntoIter;

use std::str::Chars;
use std::iter::Peekable;

use super::common::{read_group, skip_comma, read_string_literal, parse_escape_sequence, set_span};

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

fn parse_scanf_pattern(node: &TokenTree, fmt: &mut Peekable<Chars>) -> Pattern {
    match fmt.next() {
        None => {
            node.span().error("conversion lacks type at end of format").emit();
            panic!();
        },
        Some('c') => Pattern::Char,
        Some('b') => Pattern::Unsigned(2),
        Some('o') => Pattern::Unsigned(8),
        Some('d') => Pattern::Signed(10),
        Some('x') => Pattern::Unsigned(16),
        Some(c) => {
            node.span().error(format!("unknown conversion type character `{}`", c)).emit();
            panic!();
        },
    }
}

fn parse_scanf_fmt(node: &TokenTree, fmt: &mut Peekable<Chars>) -> Vec<Directive> {
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
                    Directive::Ignore(parse_scanf_pattern(node, fmt))
                } else {
                    Directive::Match(parse_scanf_pattern(node, fmt))
                }
            } else if c == '\\' {
                Directive::Exact(parse_escape_sequence(node, fmt))
            } else {
                Directive::Exact(c)
            };

        dirs.push(d);
    }

    dirs
}


pub fn parse_scanf(span: Span, iter: &mut IntoIter) -> TokenStream {
    let file = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref());
    skip_comma(span.error("scanf_impl! takes at least 2 parameters, but 1 parameters supplied"), iter);

    let node = iter.next();
    let group = read_group(span.error("unexpected end of macro invocation"), node.as_ref());
    let s = read_string_literal(node.as_ref().unwrap(), group);
    let fmt = parse_scanf_fmt(node.as_ref().unwrap(), &mut s.chars().peekable());

    let n = fmt.iter().filter(|x| match **x { Directive::Match(_) => true, _ => false }).count() + 2;
    let mut params = 2;

    let mut stream = quote!( $file );

    for d in fmt.into_iter() {
        match d {
            Directive::Whitespace => {
                stream = quote!( io::scanf::whitespace($stream) );
            },
            Directive::Exact(c) => {
                let c = TokenTree::Literal(Literal::u8_suffixed(c as _));
                stream = quote!( io::scanf::exact($stream,$c) );
            },
            Directive::Ignore(Pattern::Char) => {
                stream = quote!( io::scanf::character($stream,&mut io::scanf::Ignore) );
            },
            Directive::Ignore(Pattern::Unsigned(x)) => {
                let base = TokenTree::Literal(Literal::u8_suffixed(x));
                stream = quote!( io::scanf::unsigned($stream,&mut io::scanf::Ignore,$base) );
            },
            Directive::Ignore(Pattern::Signed(x)) => {
                let base = TokenTree::Literal(Literal::u8_suffixed(x));
                stream = quote!( io::scanf::signed($stream,&mut io::scanf::Ignore,$base) );
            },
            Directive::Match(Pattern::Char) => {
                skip_comma(span.error(format!("scanf! takes {} parameters, but {} parameters supplied", n, params)), iter);
                let param = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref());
                params += 1;
                stream = quote!( io::scanf::character($stream,&mut io::scanf::CharPattern::converter($param)) );
            },
            Directive::Match(Pattern::Unsigned(x)) => {
                skip_comma(span.error(format!("scanf! takes {} parameters, but {} parameters supplied", n, params)), iter);
                let param = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref());
                params += 1;
                let base = TokenTree::Literal(Literal::u8_suffixed(x));
                stream = quote!( io::scanf::unsigned($stream, &mut io::scanf::UnsignedPattern::converter($param, $base), $base) );
            },
            Directive::Match(Pattern::Signed(x)) => {
                skip_comma(span.error(format!("scanf! takes {} parameters, but {} parameters supplied", n, params)), iter);
                let param = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref());
                params += 1;
                let base = TokenTree::Literal(Literal::u8_suffixed(x));
                stream = quote!( io::scanf::signed($stream,&mut io::scanf::SignedPattern::converter($param, $base),$base) );
            },
        }
    }

    set_span(span, stream)
}
