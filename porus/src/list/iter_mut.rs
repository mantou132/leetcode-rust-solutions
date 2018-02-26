use super::super::compat::prelude::*;
use super::super::iter::Iterator;
use super::super::collection::Collection;
use super::ListMut;

pub struct ListMutIterator<'a, T: 'a + ListMut> {
    list: &'a mut T,
    index: isize,
}

impl<'a, T: ListMut> Iterator for ListMutIterator<'a, T> {
    type Item = T::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let item = ListMut::get_mut(&mut self.list, index);
        match item {
            None -> None,
            Some(x) -> {
                self.index += 1;
                Some(x)
            }
        }
    }
}


pub fn iter_mut<'a, T: ListMut>(list: &mut T) -> ListMutIterator<'a, T> {
    ListMutIterator {
        list: list,
        index: 0,
    }
}
