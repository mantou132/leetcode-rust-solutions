use proc_macro2::{Span, TokenStream};
use syn::buffer;
use syn::buffer::TokenBuffer;
use syn::punctuated::Punctuated;
use syn::synom::{ParseError, Synom};
use syn::token::{Comma, Paren};
use syn::{Expr, ExprTuple, LitStr};

pub struct Cursor<'a> {
    pub cur: buffer::Cursor<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(buf: &'a TokenBuffer) -> Self {
        Cursor { cur: buf.begin() }
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

pub fn parse_args(tokens: TokenStream) -> Result<(LitStr, Expr), ParseError> {
    let buf = TokenBuffer::new2(tokens.into());
    let mut cur = Cursor::new(&buf);
    let s: LitStr = cur.parse()?;

    if !cur.eof() {
        let _: Comma = cur.parse()?;
    }

    let (args, _) = Punctuated::parse_separated(cur.cur)?;

    let tuple = Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: Paren(Span::call_site()),
        elems: args,
    });

    Ok((s, tuple))
}
