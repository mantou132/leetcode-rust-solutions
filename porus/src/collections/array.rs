use core::ptr::drop_in_place;
use core::marker::PhantomData;
use super::super::traits::{CapacityPolicy, FixedCapacityPolicy, ResizingPolicy, ExactSizeIterator, Collection, Bounded, Unbounded, List, ListMut, Stack};
use super::super::storage::Chunk;
use super::DefaultResizingPolicy;


pub struct Array<T, Policy : CapacityPolicy = DefaultResizingPolicy> {
    size: usize,
    data: Chunk<T>,
    _policy: PhantomData<Policy>,
}


impl<T, P : CapacityPolicy> Array<T, P> {

    pub fn new_with_capacity(capacity: usize) -> Self {
        Array {
            size: 0,
            data: Chunk::new_with_capacity(P::initial(capacity)),
            _policy: PhantomData,
        }
    }

    pub fn new<Iter: ExactSizeIterator<Item=T>>(iter: Iter) -> Self {
        let mut array = Self::new_with_capacity(iter.len());
        for item in iter {
            Stack::push(&mut array, item);
        }
        array
    }
}


impl<T, P : CapacityPolicy> Collection for Array<T,P> {
    fn size(&self) -> usize {
        self.size
    }
}


impl<T, P : FixedCapacityPolicy> Bounded for Array<T,P> {
    fn capacity(&self) -> usize {
        self.data.capacity()
    }
}


impl<T, P : ResizingPolicy> Unbounded for Array<T,P> {
    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn reserve(&mut self, capacity: usize) {
        if capacity > self.data.capacity() {
            self.data.resize(capacity);
        }
    }

    fn shrink_to_fit(&mut self) {
        let capacity = P::initial(self.size);
        if capacity < self.data.capacity() {
            self.data.resize(capacity);
        }
    }
}


impl<T, P : CapacityPolicy> List for Array<T,P> {
    type Item = T;

    fn get(&self, index: usize) -> &T {
        if index >= self.size {
            #[cfg(debug_assertions)]
            abort!("index out of range");
        }

        self.data.get(index)
    }
}


impl<T, P : CapacityPolicy> ListMut for Array<T,P> {

    fn get_mut(&mut self, index: usize) -> &mut T {
        if index >= self.size {
            #[cfg(debug_assertions)]
            abort!("index out of range");
        }

        self.data.get_mut(index)
    }
}


impl<T, P : CapacityPolicy> Stack for Array<T,P> {
    type Item = T;

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn top(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.get(self.size - 1))
        }
    }

    default fn push(&mut self, item: T) {
        if self.size >= self.data.capacity() {
            #[cfg(debug_assertions)]
            abort!("overflow");
        }

        self.data.write(self.size, item);
        self.size += 1;
    }

    default fn pop(&mut self) -> T {
        if self.is_empty() {
            #[cfg(debug_assertions)]
            abort!("empty");
        }

        self.size -= 1;
        self.data.read(self.size)
    }
}


impl<T, P : ResizingPolicy> Stack for Array<T,P> {
    fn push(&mut self, item: T) {
        if self.size == self.data.capacity() {
            self.data.resize(P::grow(self.size))
        }

        self.data.write(self.size, item);
        self.size += 1;
    }

    fn pop(&mut self) -> T {
        if self.is_empty() {
            #[cfg(debug_assertions)]
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
            unsafe {
                drop_in_place(self.data.get_mut(i) as *mut _);
            }
        }
    }
}
