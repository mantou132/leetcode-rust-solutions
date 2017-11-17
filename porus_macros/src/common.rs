use proc_macro::{TokenStream, TokenTree, TokenTreeIter, TokenNode, Delimiter, Diagnostic};
use syntax::parse::token::{Token, Lit};
use syntax_pos::symbol::InternedString;

use std::mem::transmute;
use std::str::Chars;
use std::iter::Peekable;


pub fn read_group(error: Diagnostic, node: Option<&TokenTree>) -> Result<TokenStream, ()> {
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

pub fn skip_comma(error: Diagnostic, iter: &mut TokenTreeIter) -> Result<(), ()> {
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

pub fn read_string_literal(node: &TokenTree, stream: TokenStream) -> Result<InternedString, ()> {
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


pub fn read_term(error: Diagnostic, node: Option<&TokenTree>) -> Result<TokenTree, ()> {
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

pub fn parse_escape_sequence(node: &TokenTree, fmt: &mut Peekable<Chars>) -> Result<char, ()> {
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
