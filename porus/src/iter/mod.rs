use super::compat::prelude::*;

#[macro_export]
macro_rules! foreach {
  ( $var:ident in $iter:expr => $($expr:expr;)+) => (
      {
          let mut iter = $iter;
          while let Some($var) = iter.next() {
              $($expr;)+
          }
      }
  )
}

pub trait IterMut {
    type Item;

    fn next(&mut self) -> Option<&mut Self::Item>;
}
