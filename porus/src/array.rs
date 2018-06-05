use super::compat::prelude::*;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use super::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use super::chunk::Chunk;
use super::collection::Collection;
use super::list::{ListBase, ListMutBase, List, ListMut};
use super::stack::Stack;


#[derive(List, ListMut)]
pub struct Array<T, P : CapacityPolicy = DefaultCapacityPolicy> {
    size: isize,
    data: Chunk<T>,
    _policy: PhantomData<P>,
}


impl<T : Clone, P : CapacityPolicy> Array<T,P> {
    pub fn new_from_elem(x: T, size: isize) -> Self {
        let mut data = Chunk::new(P::initial(size));

        for i in 0..size {
            data.write(i, Clone::clone(&x));
        }

        Array {
            size: size,
            data: data,
            _policy: PhantomData,
        }
    }
}

impl<T, P : CapacityPolicy> Collection for Array<T,P> {
    fn size(&self) -> isize {
        self.size
    }
}

impl<T, P : CapacityPolicy> ListBase for Array<T,P> {
    type Elem = T;

    fn get(&self, index: isize) -> Option<&T> {
        if (0 <= index) && (index < self.size) {
            Some(self.data.get(index))
        } else {
            None
        }
    }
}

impl<T, P : CapacityPolicy> ListMutBase for Array<T,P> {
    fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        if (0 <= index) && (index < self.size) {
            Some(self.data.get_mut(index))
        } else {
            None
        }
    }
}

impl<T, P : CapacityPolicy> Stack for Array<T,P> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, elem: T) {
        if self.size == self.data.capacity() {
            self.data.resize(P::grow(self.size))
        }

        self.data.write(self.size, elem);
        self.size += 1;
    }

    fn pop(&mut self) -> T {
        if self.is_empty() {
            abort!("empty");
        }

        self.size -= 1;
        let item = self.data.read(self.size);

        let capacity = self.data.capacity();
        let new_capacity = P::shrink(self.size, capacity);

        if new_capacity < capacity {
            self.data.resize(new_capacity);
        }

        item
    }
}

impl<T, P : CapacityPolicy> Drop for Array<T,P>{
    fn drop(&mut self){
        for i in 0..self.size {
            self.data.read(i);
        }
    }
}


#[macro_export]
macro_rules! array {
    ($elem:expr; $n:expr) => (
        &mut $crate::array::Array::<_, $crate::capacity::DefaultCapacityPolicy>::new_from_elem($elem, $n)
    );
}
