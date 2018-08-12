use super::collection::Collection;
use super::list::{ListBase, List};

#[derive(List)]
pub struct StaticArray<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T: 'a> StaticArray<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        StaticArray {
            slice: slice
        }
    }
}

impl<'a, T: 'a> Collection for StaticArray<'a, T> {
    fn size(&self) -> isize {
        self.slice.len() as isize
    }
}

impl<'a, T: 'a> ListBase for StaticArray<'a, T> {
    type Elem = T;

    fn get(&self, index: isize) -> Option<&T> {
        self.slice.get(index as usize)
    }
}

#[macro_export]
macro_rules! static_array {
    ($($arg:tt)*) => (
        &$crate::static_array::StaticArray::new(&[$($arg)*])
    );
}
