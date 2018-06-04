use super::compat::prelude::*;

#[macro_export]
macro_rules! foreach {
  ( $var:ident in $iter:expr => $($stmt:stmt;)+) => (
      {
          let mut iter = $iter;
          while let Some($var) = iter.next() {
              $($stmt;)+
          }
      }
  )
}

pub trait IterMut {
    type Item;

    fn next(&mut self) -> Option<&mut Self::Item>;
}
