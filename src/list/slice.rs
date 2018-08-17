use super::super::collection::Collection;
use super::super::range::Range;
use super::{List, ListBase, ListMut, ListMutBase};

fn slice(size: isize, range: &Range) -> (isize, isize, isize) {
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

        (
            start,
            if stop <= start {
                0
            } else {
                (stop - start - 1) / step + 1
            },
            step,
        )
    } else if step < 0 {
        if !((start >= -1) && (start < size)) {
            panic!("start must in [-1,size)");
        }

        if !((stop >= -1) && (stop < size)) {
            panic!("stop must in [-1,size)");
        }

        (
            start,
            if stop >= start {
                0
            } else {
                (stop - start + 1) / step + 1
            },
            step,
        )
    } else {
        panic!("step must not be 0");
    }
}

#[derive(List)]
pub struct ListView<'a, T: 'a + List> {
    list: &'a T,
    offset: isize,
    size: isize,
    step: isize,
}

impl<'a, T: List> Collection for ListView<'a, T> {
    fn size(&self) -> isize {
        self.size
    }
}

impl<'a, T: List> ListBase for ListView<'a, T> {
    type Elem = T::Elem;

    fn get(&self, index: isize) -> Option<&Self::Elem> {
        if index < self.size {
            ListBase::get(self.list, self.offset + self.step * index)
        } else {
            None
        }
    }
}

pub trait Slice<'a, T: List + Collection> {
    fn new(&'a self, range: &Range) -> ListView<'a, T>;
}

impl<'a, 'b: 'a, T: List + Collection> Slice<'b, T> for ListView<'a, T> {
    fn new(&'b self, range: &Range) -> ListView<'b, T> {
        let (offset, size, step) = slice(Collection::size(self), range);

        ListView {
            list: self.list,
            offset: self.offset + offset * self.step,
            size,
            step: self.step * step,
        }
    }
}

impl<'a, T: List + Collection> Slice<'a, T> for T {
    fn new(&'a self, range: &Range) -> ListView<'a, T> {
        let (offset, size, step) = slice(Collection::size(self), range);
        ListView {
            list: self,
            offset,
            size,
            step,
        }
    }
}

#[macro_export]
macro_rules! slice {
    ($list:expr, [ $($arg:tt)+ ]) => {
        &$crate::list::slice::Slice::new($list, range!($($arg)+))
    }
}

#[derive(List, ListMut)]
pub struct ListMutView<'a, T: 'a + ListMut> {
    list: &'a mut T,
    offset: isize,
    size: isize,
    step: isize,
}

impl<'a, T: ListMut> Collection for ListMutView<'a, T> {
    fn size(&self) -> isize {
        self.size
    }
}

impl<'a, T: ListMut> ListBase for ListMutView<'a, T> {
    type Elem = T::Elem;

    fn get(&self, index: isize) -> Option<&Self::Elem> {
        if index < self.size {
            ListBase::get(self.list, self.offset + self.step * index)
        } else {
            None
        }
    }
}

impl<'a, T: ListMut> ListMutBase for ListMutView<'a, T> {
    fn get_mut(&mut self, index: isize) -> Option<&mut Self::Elem> {
        if index < self.size {
            ListMutBase::get_mut(self.list, self.offset + self.step * index)
        } else {
            None
        }
    }
}

pub trait SliceMut<'a, T: ListMut + Collection> {
    fn new(&'a mut self, range: &Range) -> ListMutView<'a, T>;
}

impl<'a, 'b: 'a, T: ListMut + Collection> SliceMut<'b, T> for ListMutView<'a, T> {
    fn new(&'b mut self, range: &Range) -> ListMutView<'b, T> {
        let (offset, size, step) = slice(Collection::size(self), range);

        ListMutView {
            list: self.list,
            offset: self.offset + offset * self.step,
            size,
            step: self.step * step,
        }
    }
}

impl<'a, T: ListMut + Collection> SliceMut<'a, T> for T {
    fn new(&'a mut self, range: &Range) -> ListMutView<'a, T> {
        let (offset, size, step) = slice(Collection::size(self), range);
        ListMutView {
            list: self,
            offset,
            size,
            step,
        }
    }
}

#[macro_export]
macro_rules! slice_mut {
    ($list:expr, [ $($arg:tt)+ ]) => {
        &mut $crate::list::slice::SliceMut::new($list, range!($($arg)+))
    }
}
