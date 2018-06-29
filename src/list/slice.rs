use super::super::range::Range;
use super::super::collection::Collection;
use super::{ListBase, ListMutBase, List, ListMut};


#[derive(List)]
pub struct ListView<'a, T: 'a + List> {
    list: &'a T,
    offset: isize,
    size: isize,
    step: isize,
}

impl<'a, T : List> Collection for ListView<'a, T> {
    fn size(&self) -> isize {
        self.size
    }
}

impl<'a, T : List> ListBase for ListView<'a, T> {
    type Elem = T::Elem;

    fn get(&self, index: isize) -> Option<&Self::Elem> {
        ListBase::get(self.list, self.offset + self.step * index)
    }
}

impl<'a, T : 'a + List + Collection> ListView<'a, T> {
    pub fn new(list: &'a T, range: &Range) -> Self {
        let size = Collection::size(list);
        let start = range.start(size);
        let stop = range.stop(size);
        let step = range.step();

        if step > 0 {
            if !((start >= 0) && (start <= size)) {
                panic!("start must in [0,size]");
        }

            if !((stop >= 0) && (stop <= size)) {
                panic!("stop must in [0,size]");
            }

            ListView {
                list: list,
                offset: start,
                size: if stop <= start { 0 } else { (stop - start - 1) / step + 1 },
                step: step,
            }
        } else if step < 0 {
            if !((start >= -1) && (start < size)) {
                panic!("start must in [-1,size)");
            }

            if !((stop >= -1) && (stop < size)) {
                panic!("stop must in [-1,size)");
            }

            ListView {
                list: list,
                offset: start,
                size: if stop >= start { 0 } else { (stop - start + 1) / step + 1 },
                step: step,
            }
        } else {
            unreachable!();
        }
    }
}

#[macro_export]
macro_rules! slice {
    ($list:expr, [ $($arg:tt)+ ]) => {
        &$crate::list::slice::ListView::new($list, range!($($arg)+))
    }
}

#[derive(List, ListMut)]
pub struct ListMutView<'a, T: 'a + ListMut> {
    list: &'a mut T,
    offset: isize,
    size: isize,
    step: isize,
}

impl<'a, T : ListMut> Collection for ListMutView<'a, T> {
    fn size(&self) -> isize {
        self.size
    }
}

impl<'a, T : ListMut> ListBase for ListMutView<'a, T> {
    type Elem = T::Elem;

    fn get(&self, index: isize) -> Option<&Self::Elem> {
        ListBase::get(self.list, self.offset + self.step * index)
    }
}

impl<'a, T : ListMut> ListMutBase for ListMutView<'a, T> {
    fn get_mut(&mut self, index: isize) -> Option<&mut Self::Elem> {
        ListMutBase::get_mut(self.list, self.offset + self.step * index)
    }
}

impl<'a, T : 'a + ListMut + Collection> ListMutView<'a, T> {
    pub fn new(list: &'a mut T, range: &Range) -> Self {
        let size = Collection::size(list);
        let start = range.start(size);
        let stop = range.stop(size);
        let step = range.step();

        if step > 0 {
            if !((start >= 0) && (start <= size)) {
                panic!("start must in [0,size]");
        }

            if !((stop >= 0) && (stop <= size)) {
                panic!("stop must in [0,size]");
            }

            ListMutView {
                list: list,
                offset: start,
                size: if stop <= start { 0 } else { (stop - start - 1) / step + 1 },
                step: step,
            }
        } else if step < 0 {
            if !((start >= -1) && (start < size)) {
                panic!("start must in [-1,size)");
            }

            if !((stop >= -1) && (stop < size)) {
                panic!("stop must in [-1,size)");
            }

            ListMutView {
                list: list,
                offset: start,
                size: if stop >= start { 0 } else { (stop - start + 1) / step + 1 },
                step: step,
            }
        } else {
            unreachable!();
        }
    }
}

#[macro_export]
macro_rules! slice_mut {
    ($list:expr, [ $($arg:tt)+ ]) => {
        &mut $crate::list::slice::ListMutView::new($list, range!($($arg)+))
    }
}
