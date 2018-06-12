use super::super::iter::{Iter, IterRef, IterRefMut};
use super::{ListBase, ListMutBase, List, ListMut};

pub struct ListIter<'a, T: 'a + List>
    where T::Elem : Copy {
    list: &'a T,
    index: isize,
}

impl<'a, T: 'a + List> Iter for ListIter<'a, T>
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

pub fn iter<T: List>(list: &T) -> ListIter<T> 
    where T::Elem : Copy {
    ListIter {
        list: list,
        index: 0,
    }
}


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
