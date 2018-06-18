use proc_macro::{TokenStream, TokenTree, Span, Group};
use syn::buffer::TokenBuffer;
use std::iter::FromIterator;
use syn::synom::{Synom, ParseError};
use syn::{Expr, LitStr};
use syn::token::Comma;
use syn::buffer;

pub struct Cursor<'a> {
    cur: buffer::Cursor<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(buf: &'a TokenBuffer) -> Self {
        Cursor {
            cur: buf.begin()
        }
    }

    pub fn parse<T: Synom>(&mut self) -> Result<T, ParseError> {
        let (v, cur) = T::parse(self.cur)?;
        self.cur = cur;
        Ok(v)
    }

    pub fn eof(&self) -> bool {
        self.cur.eof()
    }
}

pub fn parse_args(tokens: TokenStream) -> Result<(LitStr, Vec<Expr>), ParseError> {
    let buf = TokenBuffer::new2(tokens.into());
    let mut cur = Cursor::new(&buf);
    let s : LitStr = cur.parse()?;
    let mut exprs = Vec::new();

    while !(cur.eof()) {
        let _ : Comma = cur.parse()?;
        let arg : Expr = cur.parse()?;
        exprs.push(arg);
    }

    Ok((s, exprs))
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
