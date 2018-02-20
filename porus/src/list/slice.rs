use super::super::compat::prelude::*;
use std::ops::Index;
use super::super::range::Range;
use super::super::collection::Collection;
use super::List;


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

impl<'a, T : List> Index<isize> for ListView<'a, T> {
    type Output = T::Output;

    fn index(&self, index: isize) -> &Self::Output {
        List::get(self, index).unwrap()
    }
}

impl<'a, T : List> List for ListView<'a, T> {
    fn get(&self, index: isize) -> Option<&Self::Output> {
        self.list.get(self.offset + self.step * index)
    }
}


pub fn slice<'a, T: List>(list: &'a T, range: &Range) -> ListView<'a, T>  {
    let size = Collection::size(list);
    let start = range.start(size);
    let stop = range.stop(size);
    let step = range.step();

    if step > 0 {
        if !((start >= 0) && (start <= size)) {
            abort!("start must in [0,size]");
        }

        if !((stop >= 0) && (stop <= size)) {
            abort!("stop must in [0,size]");
        }

        ListView {
            list: list,
            offset: start,
            size: if stop <= start { 0 } else { (stop - start - 1) / step + 1 },
            step: step,
        }
    } else if step < 0 {
        if !((start >= -1) && (start < size)) {
            abort!("start must in [-1,size)");
        }

        if !((stop >= -1) && (stop < size)) {
            abort!("stop must in [-1,size)");
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
