use proc_macro::{TokenStream, TokenTree, Diagnostic, Span, Group};
use proc_macro::token_stream::IntoIter;
use syntax::parse::token::Lit;
use syntax::ast::Name;
use syntax_pos::symbol::LocalInternedString;

use std::mem::transmute;
use std::str::Chars;
use std::iter::Peekable;
use std::iter::FromIterator;


pub struct Literal {
    lit: Lit,
    suffix: Option<Name>,
    span: Span,
}


pub fn read_group(error: Diagnostic, node: Option<&TokenTree>) -> TokenTree {
    match node {
        None => {
            error.emit();
            panic!();
        },
        Some(tree) => {
            tree.clone()
        }
    }
}

pub fn skip_comma(error: Diagnostic, iter: &mut IntoIter) {
    match iter.next() {
        None => {
            error.emit();
            panic!();
        },
        Some(TokenTree::Punct(p)) => {
            match p.as_char() {
                ',' => {
                    return
                },
                c => {
                    p.span().error(format!("unexpected punctuation `{}`", c)).emit();
                    panic!();
                }
            }
        },
        Some(tree) => {
            tree.span().error(format!("expected `,`, found `{}`", tree)).emit();
            panic!();
        }
    }
}

pub fn read_string_literal(node: &TokenTree, tree: TokenTree) -> LocalInternedString {
    if let TokenTree::Literal(x) = tree {
        if let Literal{lit: Lit::Str_(s), suffix: _, span: _} = unsafe { transmute(x) } {
            return s.as_str();
        }
    }

    node.span().error(format!("expected string literal, found `{}`", node)).emit();
    panic!();
}

pub fn parse_escape_sequence(node: &TokenTree, fmt: &mut Peekable<Chars>) -> char {
    match fmt.next() {
        None => {
            node.span().error("`\\` at end of string").emit();
            panic!();
        },
        Some(c) if (c == '\'') || (c == '"') || (c == '\\') => {
            c
        },
        Some('?') => char::from(0x3f),
        Some('a') => char::from(0x07),
        Some('b') => char::from(0x08),
        Some('t') => char::from(0x09),
        Some('n') => char::from(0x0a),
        Some('v') => char::from(0x0b),
        Some('f') => char::from(0x0c),
        Some('r') => char::from(0x0d),
        Some(c) => {
            node.span().error(format!("unknown escape sequence `\\{}`", c)).emit();
            panic!();
        }
    }
}

pub fn set_span(span: Span, stream: TokenStream) -> TokenStream {
    let iter = stream.into_iter().map(|mut tree| {
        match tree {
            TokenTree::Group(g) => {
                TokenTree::Group(Group::new(g.delimiter(), set_span(span, g.stream())))
            }
            _ => {
                tree.set_span(span);
                tree
            }
        }
    });
    TokenStream::from_iter(iter)
}
