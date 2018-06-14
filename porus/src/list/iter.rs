use super::super::iter::{Iter, IterRef, IterRefMut};
use super::{ListBase, ListMutBase};

pub struct ListIter<'a, T: 'a + ListBase>
    where T::Elem : Copy {
    list: &'a T,
    index: isize,
}

impl<'a, T: 'a + ListBase> Iter for ListIter<'a, T>
    where T::Elem : Copy {
    type Item = T::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let it = ListBase::get(self.list, index);
        self.index += 1;
        match it {
            None => None,
            Some(x) => Some(*x),
        }
    }
}

pub fn iter<T: ListBase>(list: &T) -> ListIter<T>
    where T::Elem : Copy {
    ListIter {
        list: list,
        index: 0,
    }
}


pub struct ListIterRef<'a, T: 'a + ListBase> {
    list: &'a T,
    index: isize,
}

impl<'a, T: 'a + ListBase> IterRef for ListIterRef<'a, T> {
    type Item = T::Elem;

    fn next(&mut self) -> Option<&Self::Item> {
        let index = self.index;
        let it = ListBase::get(self.list, index);
        self.index += 1;
        it
    }
}

pub fn iter_ref<T: ListBase>(list: &T) -> ListIterRef<T> {
    ListIterRef {
        list: list,
        index: 0,
    }
}


pub struct ListIterRefMut<'a, T: 'a + ListMutBase> {
    list: &'a mut T,
    index: isize,
}

impl<'a, T: 'a + ListMutBase> IterRefMut for ListIterRefMut<'a, T> {
    type Item = T::Elem;

    fn next(&mut self) -> Option<&mut Self::Item> {
        let index = self.index;
        let it = ListMutBase::get_mut(self.list, index);
        self.index += 1;
        it
    }
}

pub fn iter_ref_mut<T: ListMutBase>(list: &mut T) -> ListIterRefMut<T> {
    ListIterRefMut {
        list: list,
        index: 0,
    }
}
