use super::compat::prelude::*;
use super::collection::Collection;
use std::ops::{Index, IndexMut};

pub trait ListBase : Collection {
    type Elem;

    fn get(&self, index: isize) -> Option<&Self::Elem>;
}

pub trait ListMutBase : ListBase {
    fn get_mut(&mut self, index: isize) -> Option<&mut Self::Elem>;
}

pub trait List : ListBase + Index<isize, Output=<Self as ListBase>::Elem> {
}

pub trait ListMut : ListMutBase + IndexMut<isize, Output=<Self as ListBase>::Elem> {
}


pub fn get<T: List>(list: &T, index: isize) -> Option<&T::Elem> {
    ListBase::get(list, index)
}

pub fn get_mut<T: ListMut>(list: &mut T, index: isize) -> Option<&mut T::Elem> {
    ListMutBase::get_mut(list, index)
}


mod slice;
pub use self::slice::slice;

mod iter_mut;
pub use self::iter_mut::iter_mut;
