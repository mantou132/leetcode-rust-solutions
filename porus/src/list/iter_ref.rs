use super::super::iter::IterRef;
use super::{ListBase, List};

pub struct ListIterRef<'a, T: 'a + List> {
    list: &'a T,
    index: isize,
}

impl<'a, T: 'a + List> IterRef for ListIterRef<'a, T> {
    type Item = T::Elem;

    fn next(&mut self) -> Option<&Self::Item> {
        let index = self.index;
        let it = ListBase::get(self.list, index);
        self.index += 1;
        it
    }
}

pub fn iter_ref<T: List>(list: &T) -> ListIterRef<T> {
    ListIterRef {
        list: list,
        index: 0,
    }
}
