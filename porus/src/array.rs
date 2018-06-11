use core::marker::PhantomData;
use super::ptr::{read, write, get, get_mut};
use super::alloc::{Allocator, allocate, deallocate, reallocate};
use super::os::OSAllocator;
use super::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use super::collection::Collection;
use super::list::{ListBase, ListMutBase, List, ListMut};
use super::stack::Stack;


#[derive(List, ListMut)]
pub struct Array<T, P : CapacityPolicy = DefaultCapacityPolicy, A : Allocator = OSAllocator> {
    size: isize,
    capacity: isize,
    data: *mut T,
    allocator: A,
    _policy: PhantomData<P>,
}


impl<T : Clone, P : CapacityPolicy, A : Allocator + Default> Array<T,P,A> {
    pub fn new_from_elem(x: T, size: isize) -> Self {
        let mut allocator = Default::default();
        let capacity = P::initial(size);
        let data = allocate(&mut allocator, capacity);

        for i in 0..size {
            write(data, i, Clone::clone(&x));
        }

        Array {
            size: size,
            capacity: capacity,
            data: data,
            allocator: allocator,
            _policy: PhantomData,
        }
    }
}

impl<T, P : CapacityPolicy, A : Allocator> Collection for Array<T,P,A> {
    fn size(&self) -> isize {
        self.size
    }
}

impl<T, P : CapacityPolicy, A : Allocator> ListBase for Array<T,P,A> {
    type Elem = T;

    fn get(&self, index: isize) -> Option<&T> {
        if (0 <= index) && (index < self.size) {
            Some(get(self.data, index))
        } else {
            None
        }
    }
}

impl<T, P : CapacityPolicy, A : Allocator> ListMutBase for Array<T,P,A> {
    fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        if (0 <= index) && (index < self.size) {
            Some(get_mut(self.data, index))
        } else {
            None
        }
    }
}

impl<T, P : CapacityPolicy, A : Allocator> Stack for Array<T,P,A> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, elem: T) {
        if self.size == self.capacity {
            self.capacity = P::grow(self.size);
            self.data = reallocate(&mut self.allocator, self.data, self.capacity)
        }

        write(self.data, self.size, elem);
        self.size += 1;
    }

    fn pop(&mut self) -> T {
        if self.is_empty() {
            panic!("empty");
        }

        self.size -= 1;
        let item = read(self.data, self.size);

        let new_capacity = P::shrink(self.size, self.capacity);

        if new_capacity < self.capacity {
            self.data = reallocate(&mut self.allocator, self.data, new_capacity);
            self.capacity = new_capacity;
        }

        item
    }
}

impl<T, P : CapacityPolicy, A : Allocator> Drop for Array<T,P,A>{
    fn drop(&mut self){
        for i in 0..self.size {
            read(self.data, i);
        }
        deallocate(&mut self.allocator, self.data);
    }
}


#[macro_export]
macro_rules! array {
    ($elem:expr; $n:expr) => (
        &mut $crate::array::Array::<_, $crate::capacity::DefaultCapacityPolicy>::new_from_elem($elem, $n)
    );
}
