use proc_macro::{quote, TokenStream, TokenTree, TokenTreeIter, Span, TokenNode, Literal};

use std::str::Chars;
use std::iter::Peekable;

use super::common::{read_group, skip_comma, read_string_literal, read_term, parse_escape_sequence};

#[derive(Debug)]
enum Directive {
    Literal(char),
    Char,
    Int(u8),
    String,
}

fn parse_printf_conversion(node: &TokenTree, fmt: &mut Peekable<Chars>) -> Result<Directive,()> {
    match fmt.next() {
        None => {
            node.span.error("conversion lacks type at end of format").emit();
            Err(())
        },
        Some('c') => Ok(Directive::Char),
        Some('d') => Ok(Directive::Int(10)),
        Some('s') => Ok(Directive::String),

        Some(c) => {
            node.span.error(format!("unknown conversion type character `{}`", c)).emit();
            Err(())
        },
    }
}

fn parse_printf_fmt(node: &TokenTree, fmt: &mut Peekable<Chars>) -> Result<Vec<Directive>,()> {
    let mut dirs = Vec::new();

    while let Some(&c) = fmt.peek() {
        fmt.next();

        let d =
            if c == '%' {
                if let Some(&'%') = fmt.peek() {
                    fmt.next();
                    Directive::Literal('%')
                } else {
                    parse_printf_conversion(node, fmt)?
                }
            } else if c == '\\' {
                Directive::Literal(parse_escape_sequence(node, fmt)?)
            } else {
                Directive::Literal(c)
            };

        dirs.push(d);
    }

    Ok(dirs)
}

pub fn parse_printf(span: Span, iter: &mut TokenTreeIter) -> Result<TokenStream,()> {
    let crate_ = read_term(span.error("printf_impl! takes at least 3 parameters, but 0 parameter supplied"), iter.next().as_ref())?;
    skip_comma(span.error("printf_impl! takes at least 3 parameters, but 1 parameter supplied"), iter)?;

    let file = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref())?;
    skip_comma(span.error("printf_impl! takes at least 3 parameters, but 2 parameters supplied"), iter)?;

    let node = iter.next();
    let group = read_group(span.error("unexpected end of macro invocation"), node.as_ref())?;
    let s = read_string_literal(node.as_ref().unwrap(), group)?;
    let fmt = parse_printf_fmt(node.as_ref().unwrap(), &mut s.chars().peekable())?;

    let n = fmt.iter().filter(|x| match **x { Directive::Literal(_) => false, _ => true }).count() + 2;
    let mut params = 2;

    let mut stream = quote!( $file );

    for d in fmt.into_iter() {
        match d {
            Directive::Literal(c) => {
                let c = TokenNode::Literal(Literal::integer(c as _));
                stream = quote!( $crate_::io::printf::write_char($stream,$c) );
            },
            Directive::Char => {
                skip_comma(span.error(format!("printf! takes {} parameters, but {} parameters supplied", n, params)), iter)?;
                let param = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref())?;
                params += 1;
                stream = quote!( $crate_::io::printf::write_char($stream,$param) );
            },
            Directive::Int(x) => {
                skip_comma(span.error(format!("printf! takes {} parameters, but {} parameters supplied", n, params)), iter)?;
                let param = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref())?;
                params += 1;
                let base = TokenNode::Literal(Literal::u8(x));
                stream = quote!( $crate_::io::printf::write_string($stream, $crate_::io::printf::IntField::converter($param,$base)) );
            },
            Directive::String => {
                skip_comma(span.error(format!("printf! takes {} parameters, but {} parameters supplied", n, params)), iter)?;
                let param = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref())?;
                params += 1;
                stream = quote!( $crate_::io::printf::write_string($stream, $param) );
            },
        }
    }

    Ok(stream)
}
