use super::alloc::{allocate, deallocate, reallocate, Allocator};
use super::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use super::collection::Collection;
use super::deque::Deque;
use super::list::{List, ListBase, ListMut, ListMutBase};
use super::os::OSAllocator;
use super::ptr::{copy, get, get_mut, read, write};
use core::marker::PhantomData;

#[derive(List, ListMut)]
pub struct Buffer<T, P: CapacityPolicy = DefaultCapacityPolicy, A: Allocator = OSAllocator> {
    front: isize,
    back: isize,
    capacity: isize,
    data: *mut T,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<T, P: CapacityPolicy, A: Allocator + Default> Buffer<T, P, A> {
    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }

    pub fn new_with_capacity(capacity: isize) -> Self {
        let capacity = P::initial(capacity) + 1;
        let mut allocator = Default::default();
        let data = allocate(&mut allocator, capacity);
        Buffer {
            front: 0,
            back: 0,
            capacity: capacity,
            data: data,
            allocator: allocator,
            _policy: PhantomData,
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Buffer<T, P, A> {
    fn increase_index(&self, index: isize) -> isize {
        if index + 1 == self.capacity {
            0
        } else {
            index + 1
        }
    }

    fn decrease_index(&self, index: isize) -> isize {
        if index == 0 {
            self.capacity - 1
        } else {
            index - 1
        }
    }

    fn grow_to(&mut self, new_capacity: isize) {
        self.data = reallocate(&mut self.allocator, self.data, new_capacity);
        if self.back < self.front {
            let grow = new_capacity - self.capacity;
            copy(
                self.data,
                self.front,
                self.front + grow,
                self.capacity - self.front,
            );
            self.front += grow;
        }
        self.capacity = new_capacity;
    }

    fn shrink_to(&mut self, new_capacity: isize) {
        if self.back < self.front {
            let shrink = self.capacity - new_capacity;
            copy(
                self.data,
                self.front,
                self.front - shrink,
                self.capacity - self.front,
            );
            self.front -= shrink;
        } else if self.back > new_capacity {
            let size = self.back - self.front;
            copy(self.data, self.front, 0, size);
            self.front = 0;
            self.back = size;
        }

        self.data = reallocate(&mut self.allocator, self.data, new_capacity);
        self.capacity = new_capacity;
    }

    fn is_full(&self) -> bool {
        self.increase_index(self.back) == self.front
    }

    fn grow(&mut self) {
        let new_capacity = P::grow(self.capacity - 1) + 1;
        self.grow_to(new_capacity);
    }

    fn shrink(&mut self) {
        let new_capacity = P::shrink(Collection::size(self), self.capacity - 1) + 1;
        if new_capacity < self.capacity {
            self.shrink_to(new_capacity);
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Collection for Buffer<T, P, A> {
    fn size(&self) -> isize {
        if self.front <= self.back {
            self.back - self.front
        } else {
            self.back + self.capacity - self.front
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> ListBase for Buffer<T, P, A> {
    type Elem = T;

    fn get(&self, index: isize) -> Option<&T> {
        if self.front <= self.back {
            if self.front + index >= self.back {
                None
            } else {
                Some(get(self.data, self.front + index))
            }
        } else {
            if self.front + index >= self.back + self.capacity {
                None
            } else if self.front + index >= self.capacity {
                Some(get(self.data, self.front + index - self.capacity))
            } else {
                Some(get(self.data, self.front + index))
            }
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> ListMutBase for Buffer<T, P, A> {
    fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        if self.front <= self.back {
            if self.front + index >= self.back {
                None
            } else {
                Some(get_mut(self.data, self.front + index))
            }
        } else {
            if self.front + index >= self.back + self.capacity {
                None
            } else if self.front + index >= self.capacity {
                Some(get_mut(self.data, self.front + index - self.capacity))
            } else {
                Some(get_mut(self.data, self.front + index))
            }
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Deque for Buffer<T, P, A> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.front == self.back
    }

    fn push_front(&mut self, elem: T) {
        if self.is_full() {
            self.grow();
        }

        self.front = self.decrease_index(self.front);
        write(self.data, self.front, elem);
    }

    fn pop_front(&mut self) -> T {
        if self.is_empty() {
            panic!("empty");
        }

        let elem = read(self.data, self.front);
        self.front = self.increase_index(self.front);
        self.shrink();
        elem
    }

    fn push_back(&mut self, elem: T) {
        if self.is_full() {
            self.grow();
        }

        write(self.data, self.back, elem);
        self.back = self.increase_index(self.back);
    }

    fn pop_back(&mut self) -> T {
        if self.is_empty() {
            panic!("empty");
        }

        self.back = self.decrease_index(self.back);
        let elem = read(self.data, self.back);
        self.shrink();
        elem
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Drop for Buffer<T, P, A> {
    fn drop(&mut self) {
        if self.back < self.front {
            for i in 0..self.back {
                read(self.data, i);
            }

            for i in self.front..self.capacity {
                read(self.data, i);
            }
        } else {
            for i in self.front..self.back {
                read(self.data, i);
            }
        }
        deallocate(&mut self.allocator, self.data);
    }
}

#[macro_export]
macro_rules! buffer {
    () => {
        &mut $crate::buffer::Buffer::<_, $crate::capacity::DefaultCapacityPolicy>::new()
    };
}
