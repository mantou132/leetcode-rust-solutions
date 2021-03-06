use super::alloc::{allocate, deallocate, reallocate, Allocator};
use super::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use super::collection::Collection;
use super::list::{List, ListBase, ListMut, ListMutBase};
use super::os::OSAllocator;
use super::ptr::{get, get_mut, read, write};
use super::stack::Stack;
use core::iter::{ExactSizeIterator, Iterator};
use core::marker::PhantomData;

#[derive(List, ListMut)]
pub struct Array<T, P: CapacityPolicy = DefaultCapacityPolicy, A: Allocator = OSAllocator> {
    size: isize,
    capacity: isize,
    data: *mut T,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<T, P: CapacityPolicy, A: Allocator + Default> Array<T, P, A> {
    pub fn new_from_iter<I: ExactSizeIterator<Item = T>>(mut it: I) -> Self {
        let size = ExactSizeIterator::len(&it) as isize;
        let mut allocator = Default::default();
        let capacity = P::initial(size);
        let data = unsafe { allocate(&mut allocator, capacity) };

        for i in 0..size {
            unsafe { write(data, i, Iterator::next(&mut it).unwrap()) }
        }

        Array {
            size,
            capacity,
            data,
            allocator,
            _policy: PhantomData,
        }
    }
}

impl<T: Clone, P: CapacityPolicy, A: Allocator + Default> Array<T, P, A> {
    pub fn new_from_elem(x: T, size: isize) -> Self {
        Array::new_from_iter((0..size).map(|_| Clone::clone(&x)))
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Collection for Array<T, P, A> {
    fn size(&self) -> isize {
        self.size
    }
}

impl<T, P: CapacityPolicy, A: Allocator> ListBase for Array<T, P, A> {
    type Elem = T;

    fn get(&self, index: isize) -> Option<&T> {
        if (0 <= index) && (index < self.size) {
            Some(unsafe { get(self.data, index) })
        } else {
            None
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> ListMutBase for Array<T, P, A> {
    fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        if (0 <= index) && (index < self.size) {
            Some(unsafe { get_mut(self.data, index) })
        } else {
            None
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Stack for Array<T, P, A> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, elem: T) {
        if self.size == self.capacity {
            self.capacity = P::grow(self.size);
            self.data = unsafe { reallocate(&mut self.allocator, self.data, self.capacity) };
        }

        unsafe { write(self.data, self.size, elem) };
        self.size += 1;
    }

    fn pop(&mut self) -> T {
        if self.is_empty() {
            panic!("empty");
        }

        self.size -= 1;
        let item = unsafe { read(self.data, self.size) };

        let new_capacity = P::shrink(self.size, self.capacity);

        if new_capacity < self.capacity {
            self.data = unsafe { reallocate(&mut self.allocator, self.data, new_capacity) };
            self.capacity = new_capacity;
        }

        item
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Drop for Array<T, P, A> {
    fn drop(&mut self) {
        for i in 0..self.size {
            unsafe { read(self.data, i) };
        }
        unsafe { deallocate(&mut self.allocator, self.data) };
    }
}

#[macro_export]
macro_rules! array {
    ($elem:expr; $n:expr) => {
        &mut $crate::array::Array::<_>::new_from_elem($elem, $n)
    };
}

#[cfg(test)]
mod tests {
    use super::super::tests::Item;
    use super::Array;
    use core::cell::RefCell;

    #[test]
    fn test_drop() {
        let counter = RefCell::new(0);
        {
            Array::<_>::new_from_iter((0..10).map(|_| Item::new(&counter)));
        }
        assert!(counter.into_inner() == 10);
    }
}
