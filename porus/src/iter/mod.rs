use super::compat::prelude::*;

pub trait IterMut {
    type Item;

    fn next(&mut self) -> Option<&mut Self::Item>;
}

pub macro foreach( $var:ident in $iter:expr => $($expr:expr;)+) {
    {
        let mut iter = $iter;
        while let Some($var) = iter.next() {
            $($expr;)+
        }
    }
}
