use super::compat::prelude::*;
use std::ops::{Index, IndexMut};

pub trait List : Index<isize> {
    fn get(&self, index: isize) -> Option<&Self::Output>;
}


pub trait ListMut : List + IndexMut<isize> {
    fn get_mut(&mut self, index: isize) -> Option<&mut Self::Output>;
}
