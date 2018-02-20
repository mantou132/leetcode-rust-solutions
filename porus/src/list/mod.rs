use super::compat::prelude::*;
use super::collection::Collection;
use std::ops::{Index, IndexMut};


pub trait List : Collection + Index<isize> {
    fn get(&self, index: isize) -> Option<&Self::Output>;
}


pub trait ListMut : List + IndexMut<isize> {
    fn get_mut(&mut self, index: isize) -> Option<&mut Self::Output>;
}


pub fn get<T: List>(list: &T, index: isize) -> Option<&T::Output> {
    List::get(list, index)
}

pub fn get_mut<T: ListMut>(list: &mut T, index: isize) -> Option<&mut T::Output> {
    ListMut::get_mut(list, index)
}


mod slice;
pub use self::slice::slice;
