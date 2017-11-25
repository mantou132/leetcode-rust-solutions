use super::compat::prelude::*;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use super::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use super::chunk::Chunk;
use super::list::{List, ListMut};


pub struct Array<T, P : CapacityPolicy = DefaultCapacityPolicy> {
    size: isize,
    data: Chunk<T>,
    _policy: PhantomData<P>,
}


impl<T : Clone, P : CapacityPolicy> Array<T,P> {
    pub fn new_from_elem(x: T, size: isize) -> Self {
        let mut data = Chunk::new(P::initial(size));

        for i in 0..size {
            Chunk::write(&mut data, i, Clone::clone(&x));
        }

        Array {
            size: size,
            data: data,
            _policy: PhantomData,
        }
    }
}


impl<T, P : CapacityPolicy> Index<isize> for Array<T,P> {
    type Output = T;

    fn index(&self, index: isize) -> &T {
        List::get(self, index).unwrap()
    }
}

impl<T, P : CapacityPolicy> IndexMut<isize> for Array<T,P> {

    fn index_mut(&mut self, index: isize) -> &mut T {
        ListMut::get_mut(self, index).unwrap()
    }
}

impl<T, P : CapacityPolicy> List for Array<T,P> {
    fn get(&self, index: isize) -> Option<&T> {
        if (0 <= index) && (index < self.size) {
            Some(Chunk::get(&self.data, index))
        } else {
            None
        }
    }
}

impl<T, P : CapacityPolicy> ListMut for Array<T,P> {
    fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        if (0 <= index) && (index < self.size) {
            Some(Chunk::get_mut(&mut self.data, index))
        } else {
            None
        }
    }
}


impl<T, P : CapacityPolicy> Drop for Array<T,P>{
    fn drop(&mut self){
        for i in 0..self.size {
            Chunk::read(&mut self.data, i);
        }
    }
}


#[macro_export]
macro_rules! array {
    ($elem:expr; $n:expr) => (
        &mut $crate::array::Array::<_, $crate::capacity::DefaultCapacityPolicy>::new_from_elem($elem, $n)
    );
}
