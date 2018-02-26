use super::compat::prelude::*;
use super::collection::Collection;
use std::ops::{Index, IndexMut};

pub trait ListBase : Collection {
    type Element: ?Sized;

    fn get(&self, index: isize) -> Option<&Self::Element>;
}

pub trait ListMutBase : ListBase {
    fn get_mut(&mut self, index: isize) -> Option<&mut Self::Element>;
}

pub trait List : ListBase + Index<isize, Output=<Self as ListBase>::Element> {
}

pub trait ListMut : ListMutBase + IndexMut<isize, Output=<Self as ListBase>::Element> {
}


pub fn get<T: List>(list: &T, index: isize) -> Option<&T::Element> {
    ListBase::get(list, index)
}

pub fn get_mut<T: ListMut>(list: &mut T, index: isize) -> Option<&mut T::Element> {
    ListMutBase::get_mut(list, index)
}


mod slice;
pub use self::slice::slice;
