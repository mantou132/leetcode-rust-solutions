use proc_macro::{TokenStream, TokenTree, Span, Group};
use std::iter::FromIterator;
use syn::synom::Synom;
use syn::synom::ParseError;
use syn::buffer;

pub struct Cursor<'a> {
    cur: buffer::Cursor<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(buf: &'a buffer::TokenBuffer) -> Self {
        Cursor {
            cur: buf.begin()
        }
    }

    pub fn parse<T: Synom>(&mut self) -> Result<T, ParseError> {
        let (v, cur) = T::parse(self.cur)?;
        self.cur = cur;
        Ok(v)
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
