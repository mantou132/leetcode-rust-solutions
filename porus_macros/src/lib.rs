#![feature(rustc_private)]
#![feature(ascii_ctype)]
#![feature(proc_macro)]

extern crate proc_macro;
extern crate syntax;
extern crate syntax_pos;

use proc_macro::{quote, TokenStream, TokenTree, TokenTreeIter, Span, TokenNode, Literal, Delimiter, Diagnostic};
use syntax::parse::token::{Token, Lit};
use syntax_pos::symbol::InternedString;

use std::mem::transmute;
use std::str::Chars;
use std::iter::Peekable;


fn read_group(error: Diagnostic, node: Option<&TokenTree>) -> Result<TokenStream, ()> {
    match node {
        None => {
            error.emit();
            Err(())
        },
        Some(&TokenTree{span: _, kind: TokenNode::Group(Delimiter::None, ref stream)}) => {
            Ok(stream.clone())
        },
        Some(ref tree) => {
            tree.span.error(format!("expected TokenNode::Group, found `{}`", tree)).emit();
            Err(())
        }
    }
}

fn skip_comma(error: Diagnostic, iter: &mut TokenTreeIter) -> Result<(), ()> {
    match iter.next() {
        None => {
            error.emit();
            Err(())
        },
        Some(TokenTree{span: _, kind: TokenNode::Op(',', _)}) => {
            Ok(())
        },
        Some(tree) => {
            tree.span.error(format!("expected `,`, found `{}`", tree)).emit();
            Err(())
        }
    }
}

fn read_string_literal(node: &TokenTree, stream: TokenStream) -> Result<InternedString, ()> {
    let iter = &mut stream.clone().into_iter();
    let first = iter.next();
    if let None = iter.next() {
        if let Some(TokenTree{span: _, kind: TokenNode::Literal(x)}) = first {
            if let Token::Literal(Lit::Str_(s), None) = unsafe { transmute(x) } {
                return Ok(s.as_str());
            }
        }
    }

    node.span.error(format!("expected string literal, found `{}`", node)).emit();
    Err(())
}

#[derive(Debug)]
enum ScanfPattern {
    Char,
    Signed(u8),
    Unsigned(u8),
}

#[derive(Debug)]
enum ScanfDirective {
    Whitespace,
    Exact(char),
    Match(ScanfPattern),
    Ignore(ScanfPattern),
}

fn parse_escape_sequence(node: &TokenTree, fmt: &mut Peekable<Chars>) -> Result<char, ()> {
    match fmt.next() {
        None => {
            node.span.error("`\\` at end of string").emit();
            Err(())
        },
        Some(c) if (c == '\'') || (c == '"') || (c == '\\') => {
            Ok(c)
        },
        Some('?') => Ok(char::from(0x3f)),
        Some('a') => Ok(char::from(0x07)),
        Some('b') => Ok(char::from(0x08)),
        Some('t') => Ok(char::from(0x09)),
        Some('n') => Ok(char::from(0x0a)),
        Some('v') => Ok(char::from(0x0b)),
        Some('f') => Ok(char::from(0x0c)),
        Some('r') => Ok(char::from(0x0d)),
        Some(c) => {
            node.span.error(format!("unknown escape sequence `\\{}`", c)).emit();
            Err(())
        }
    }
}

fn parse_scanf_pattern(node: &TokenTree, fmt: &mut Peekable<Chars>) -> Result<ScanfPattern,()> {
    match fmt.next() {
        None => {
            node.span.error("conversion lacks type at end of format").emit();
            Err(())
        },
        Some('c') => Ok(ScanfPattern::Char),
        Some('b') => Ok(ScanfPattern::Unsigned(2)),
        Some('o') => Ok(ScanfPattern::Unsigned(8)),
        Some('d') => Ok(ScanfPattern::Signed(10)),
        Some('x') => Ok(ScanfPattern::Unsigned(16)),
        Some(c) => {
            node.span.error(format!("unknown conversion type character `{}`", c)).emit();
            Err(())
        },
    }
}

fn parse_scanf_fmt(node: &TokenTree, fmt: &mut Peekable<Chars>) -> Result<Vec<ScanfDirective>,()> {
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
                ScanfDirective::Whitespace
            } else if c == '%' {
                if let Some(&'%') = fmt.peek() {
                    fmt.next();
                    ScanfDirective::Exact('c')
                } else if let Some(&'*') = fmt.peek() {
                    fmt.next();
                    ScanfDirective::Ignore(parse_scanf_pattern(node, fmt)?)
                } else {
                    ScanfDirective::Match(parse_scanf_pattern(node, fmt)?)
                }
            } else if c == '\\' {
                ScanfDirective::Exact(parse_escape_sequence(node, fmt)?)
            } else {
                ScanfDirective::Exact(c)
            };

        dirs.push(d);
    }

    Ok(dirs)
}

fn read_term(error: Diagnostic, node: Option<&TokenTree>) -> Result<TokenTree, ()> {
    match node {
        None => {
            error.emit();
            Err(())
        },
        Some(&TokenTree{span: _, kind: TokenNode::Term(_)}) => {
            Ok(node.unwrap().clone())
        },
        Some(ref tree) => {
            tree.span.error(format!("expected TokenNode::Term, found `{}`", tree)).emit();
            Err(())
        }
    }
}


fn parse_scanf(span: Span, iter: &mut TokenTreeIter) -> Result<TokenStream,()> {
    let crate_ = read_term(span.error("scanf_impl! takes at least 3 parameters, but 0 parameter supplied"), iter.next().as_ref())?;
    skip_comma(span.error("scanf_impl! takes at least 3 parameters, but 1 parameter supplied"), iter)?;

    let file = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref())?;
    skip_comma(span.error("scanf_impl! takes at least 3 parameters, but 2 parameters supplied"), iter)?;

    let node = iter.next();
    let group = read_group(span.error("unexpected end of macro invocation"), node.as_ref())?;
    let s = read_string_literal(node.as_ref().unwrap(), group)?;
    let fmt = parse_scanf_fmt(node.as_ref().unwrap(), &mut s.chars().peekable())?;

    let n = fmt.iter().filter(|x| match **x { ScanfDirective::Match(_) => true, _ => false }).count() + 2;
    let mut params = 2;

    let mut stream = quote!( $crate_::io::scanf::ok($file) );

    for d in fmt.into_iter() {
        match d {
            ScanfDirective::Whitespace => {
                stream = quote!( $stream.and_then($crate_::io::scanf::whitespace) );
            },
            ScanfDirective::Exact(c) => {
                let c = TokenNode::Literal(Literal::integer(c as _));
                stream = quote!( $stream.and_then(|s| $crate_::io::scanf::exact(s,$c)) );
            },
            ScanfDirective::Ignore(ScanfPattern::Char) => {
                stream = quote!( $stream.and_then(|s| $crate_::io::scanf::character(s,&mut $crate_::io::scanf::Ignore)) );
            },
            ScanfDirective::Ignore(ScanfPattern::Unsigned(x)) => {
                let base = TokenNode::Literal(Literal::u8(x));
                stream = quote!( $stream.and_then(|s| $crate_::io::scanf::unsigned(s,&mut $crate_::io::scanf::Ignore,$base)) );
            },
            ScanfDirective::Ignore(ScanfPattern::Signed(x)) => {
                let base = TokenNode::Literal(Literal::u8(x));
                stream = quote!( $stream.and_then(|s| $crate_::io::scanf::signed(s,&mut $crate_::io::scanf::Ignore,$base)) );
            },
            ScanfDirective::Match(ScanfPattern::Char) => {
                skip_comma(span.error(format!("scanf! takes {} parameters, but {} parameters supplied", n, params)), iter)?;
                let param = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref())?;
                params += 1;
                stream = quote!( $stream.and_then(|s| $crate_::io::scanf::character(s,&mut $crate_::io::scanf::CharPattern::converter($param))) );
            },
            ScanfDirective::Match(ScanfPattern::Unsigned(x)) => {
                skip_comma(span.error(format!("scanf! takes {} parameters, but {} parameters supplied", n, params)), iter)?;
                let param = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref())?;
                params += 1;
                let base = TokenNode::Literal(Literal::u8(x));
                stream = quote!( $stream.and_then(|s| $crate_::io::scanf::unsigned(s,&mut $crate_::io::scanf::UnsignedPattern::converter($param, $base),$base)) );
            },
            ScanfDirective::Match(ScanfPattern::Signed(x)) => {
                skip_comma(span.error(format!("scanf! takes {} parameters, but {} parameters supplied", n, params)), iter)?;
                let param = read_group(span.error("unexpected end of macro invocation"), iter.next().as_ref())?;
                params += 1;
                let base = TokenNode::Literal(Literal::u8(x));
                stream = quote!( $stream.and_then(|s| $crate_::io::scanf::signed(s,&mut $crate_::io::scanf::SignedPattern::converter($param, $base),$base)) );
            },
        }
    }

    Ok(stream)
}


#[proc_macro]
pub fn scanf_impl(stream: TokenStream) -> TokenStream {
    match parse_scanf(Span::call_site(), &mut stream.into_iter()) {
        Ok(s) => s,
        Err(()) => TokenStream::empty(),
    }
}
