use super::super::compat::prelude::*;
use super::super::iter::IterMut;
use super::{ListMutBase, ListMut};

pub struct ListIterMut<'a, T: 'a + ListMut> {
    list: &'a mut T,
    index: isize,
}

impl<'a, T: 'a + ListMut> IterMut for ListIterMut<'a, T> {
    type Item = T::Elem;

    fn next(&mut self) -> Option<&mut Self::Item> {
        let index = self.index;
        let it = ListMutBase::get_mut(self.list, index);
        self.index += 1;
        it
    }
}

pub fn iter_mut<T: ListMut>(list: &mut T) -> ListIterMut<T> {
    ListIterMut {
        list: list,
        index: 0,
    }
}
