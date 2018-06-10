use super::super::compat::prelude::*;
use super::super::iter::IterRefMut;
use super::{ListMutBase, ListMut};

pub struct ListIterRefMut<'a, T: 'a + ListMut> {
    list: &'a mut T,
    index: isize,
}

impl<'a, T: 'a + ListMut> IterRefMut for ListIterRefMut<'a, T> {
    type Item = T::Elem;

    fn next(&mut self) -> Option<&mut Self::Item> {
        let index = self.index;
        let it = ListMutBase::get_mut(self.list, index);
        self.index += 1;
        it
    }
}

pub fn iter_ref_mut<T: ListMut>(list: &mut T) -> ListIterRefMut<T> {
    ListIterRefMut {
        list: list,
        index: 0,
    }
}
