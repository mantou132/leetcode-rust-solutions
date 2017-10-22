use core::ptr::{drop_in_place, copy};
use core::marker::PhantomData;
use super::super::traits::{CapacityPolicy, FixedCapacityPolicy, ResizingPolicy, ExactSizeIterator, Collection, Bounded, Unbounded, List, ListMut, Deque};
use super::super::storage::Chunk;
use super::DefaultResizingPolicy;


pub struct Buffer<T, Policy : CapacityPolicy = DefaultResizingPolicy> {
    front: usize,
    back: usize,
    data: Chunk<T>,
    _policy: PhantomData<Policy>,
}


impl<T, P : CapacityPolicy> Buffer<T, P> {

    pub fn new_with_capacity(capacity: usize) -> Self {
        Buffer {
            front: 0,
            back: 0,
            data: Chunk::new_with_capacity(P::initial(capacity) + 1),
            _policy: PhantomData,
        }
    }

    pub fn new<Iter: ExactSizeIterator<Item=T>>(iter: Iter) -> Self {
        let mut array = Self::new_with_capacity(iter.len());
        for item in iter {
            Deque::push_back(&mut array, item);
        }
        array
    }

    fn grow_to(&mut self, new_capacity: usize) {
        let capacity = self.data.capacity();
        self.data.resize(new_capacity);
        if self.back < self.front {
            let grow = new_capacity - capacity;
            unsafe {
                copy(self.data.get_ptr(self.front), self.data.get_ptr_mut(self.front).offset(grow as isize), capacity - self.front);
            }
            self.front += grow;
        }
    }

    fn shrink_to(&mut self, new_capacity: usize) {
        let capacity = self.data.capacity();
        if self.back < self.front {
            let shrink = capacity - new_capacity;
            unsafe {
                copy(self.data.get_ptr(self.front), self.data.get_ptr_mut(self.front).offset(-(shrink as isize)), capacity - self.front);
            }
            self.front -= shrink;
        } else {
            let size = self.back - self.front;
            unsafe {
                copy(self.data.get_ptr(self.front), self.data.get_ptr_mut(0), size);
            }
            self.front = 0;
            self.back = size;
        }

        self.data.resize(new_capacity);
    }

    fn increase_index(&self, index: usize) -> usize {
        if index + 1 == self.data.capacity() {
            0
        } else {
            index + 1
        }
    }

    fn decrease_index(&self, index: usize) -> usize {
        if index == 0 {
            self.data.capacity() - 1
        } else {
            index - 1
        }
    }

    fn is_full(&self) -> bool {
        self.increase_index(self.back) == self.front
    }
}


impl<T, P : ResizingPolicy> Buffer<T,P> {
    fn grow(&mut self) {
        let capacity = self.data.capacity() - 1;
        self.grow_to(P::grow(capacity) + 1);
    }


    fn shrink(&mut self) {
        let capacity = self.data.capacity() - 1;
        let new_capacity = P::shrink(Collection::size(self), capacity);
        if new_capacity < capacity {
            self.shrink_to(new_capacity + 1);
        }
    }
}


impl<T, P : CapacityPolicy> Collection for Buffer<T,P> {
    fn size(&self) -> usize {
        if self.front <= self.back {
            self.back - self.front
        } else {
            self.back + self.data.capacity() - self.front
        }
    }
}


impl<T, P : FixedCapacityPolicy> Bounded for Buffer<T,P> {
    fn capacity(&self) -> usize {
        self.data.capacity() - 1
    }
}


impl<T, P : ResizingPolicy> Unbounded for Buffer<T,P> {
    fn capacity(&self) -> usize {
        self.data.capacity() - 1
    }

    fn reserve(&mut self, capacity: usize) {
        if capacity + 1 > self.data.capacity() {
            self.grow_to(capacity + 1);
        }
    }

    fn shrink_to_fit(&mut self) {
        let capacity = 1 + P::initial(Collection::size(self));
        if capacity < self.data.capacity() {
            self.shrink_to(capacity);
        }
    }
}


impl<T, P : CapacityPolicy> List for Buffer<T,P> {
    type Item = T;

    fn get(&self, index: usize) -> &T {
        if self.front <= self.back {
            if self.front + index >= self.back {
                #[cfg(debug_assertions)]
                abort!("index out of range");
            }

            self.data.get(self.front + index)
        } else {
            let capacity = self.data.capacity();
            if self.front + index >= self.back + capacity {
                #[cfg(debug_assertions)]
                abort!("index out of range");
            }

            if self.front + index >= capacity {
                self.data.get(self.front + index - capacity)
            } else {
                self.data.get(self.front + index)
            }
        }
    }
}


impl<T, P : CapacityPolicy> ListMut for Buffer<T,P> {

    fn get_mut(&mut self, index: usize) -> &mut T {
        if self.front <= self.back {
            if self.front + index >= self.back {
                #[cfg(debug_assertions)]
                abort!("index out of range");
            }

            self.data.get_mut(self.front + index)
        } else {
            let capacity = self.data.capacity();
            if self.front + index >= self.back + capacity {
                #[cfg(debug_assertions)]
                abort!("index out of range");
            }

            if self.front + index >= capacity {
                self.data.get_mut(self.front + index - capacity)
            } else {
                self.data.get_mut(self.front + index)
            }
        }
    }
}


impl<T, P : CapacityPolicy> Deque for Buffer<T,P> {
    type Item = T;

    fn is_empty(&self) -> bool {
        self.front == self.back
    }

    fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.get(self.front))
        }
    }

    fn back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.get(self.decrease_index(self.back)))
        }
    }

    default fn push_front(&mut self, item: T) {
        if self.is_full() {
            #[cfg(debug_assertions)]
            abort!("overflow");
        }

        self.front = self.decrease_index(self.front);
        self.data.write(self.front, item);
    }

    default fn pop_front(&mut self) -> T {
        if self.is_empty() {
            #[cfg(debug_assertions)]
            abort!("empty");
        }

        let item = self.data.read(self.front);
        self.front = self.increase_index(self.front);
        item
    }

    default fn push_back(&mut self, item: T) {
        if self.is_full() {
            #[cfg(debug_assertions)]
            abort!("overflow");
        }

        self.data.write(self.back, item);
        self.back = self.increase_index(self.back);
    }

    default fn pop_back(&mut self) -> T {
        if self.is_empty() {
            #[cfg(debug_assertions)]
            abort!("empty");
        }

        self.back = self.decrease_index(self.back);
        self.data.read(self.back)
    }
}


impl<T, P : ResizingPolicy> Deque for Buffer<T,P> {

    fn push_front(&mut self, item: T) {
        if self.is_full() {
            self.grow();
        }

        self.front = self.decrease_index(self.front);
        self.data.write(self.front, item);
    }

    fn pop_front(&mut self) -> T {
        if self.is_empty() {
            #[cfg(debug_assertions)]
            abort!("empty");
        }

        let item = self.data.read(self.front);
        self.front = self.increase_index(self.front);
        self.shrink();
        item
    }

    fn push_back(&mut self, item: T) {
        if self.is_full() {
            self.grow();
        }

        self.data.write(self.back, item);
        self.back = self.increase_index(self.back);
    }

    fn pop_back(&mut self) -> T {
        if self.is_empty() {
            #[cfg(debug_assertions)]
            abort!("empty");
        }

        self.back = self.decrease_index(self.back);
        let item = self.data.read(self.back);
        self.shrink();
        item
    }
}


impl<T, P : CapacityPolicy> Drop for Buffer<T,P>{
    fn drop(&mut self){
        if self.back < self.front {
            for i in 0..self.back {
                unsafe {
                    drop_in_place(self.data.get_mut(i) as *mut _);
                }
            }

            for i in self.front..self.data.capacity() {
                unsafe {
                    drop_in_place(self.data.get_mut(i) as *mut _);
                }
            }
        } else {
            for i in self.front..self.back {
                unsafe {
                    drop_in_place(self.data.get_mut(i) as *mut _);
                }
            }
        }
    }
}
