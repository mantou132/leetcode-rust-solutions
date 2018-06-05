use super::compat::prelude::*;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use super::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use super::chunk::Chunk;
use super::collection::Collection;
use super::list::{ListBase, ListMutBase, List, ListMut};
use super::deque::Deque;

#[derive(List, ListMut)]
pub struct Buffer<T, P : CapacityPolicy = DefaultCapacityPolicy> {
    front: isize,
    back: isize,
    data: Chunk<T>,
    _policy: PhantomData<P>,
}

impl<T, P : CapacityPolicy> Buffer<T, P> {
    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }

    pub fn new_with_capacity(capacity: isize) -> Self {
        Buffer {
            front: 0,
            back: 0,
            data: Chunk::new(P::initial(capacity) + 1),
            _policy: PhantomData,
        }
    }

    fn increase_index(&self, index: isize) -> isize {
        if index + 1 == self.data.capacity() {
            0
        } else {
            index + 1
        }
    }

    fn decrease_index(&self, index: isize) -> isize {
        if index == 0 {
            self.data.capacity() - 1
        } else {
            index - 1
        }
    }

    fn grow_to(&mut self, new_capacity: isize) {
        let capacity = self.data.capacity();
        self.data.resize(new_capacity);
        if self.back < self.front {
            let grow = new_capacity - capacity;
            self.data.copy(self.front, self.front+grow, capacity-self.front);
            self.front += grow;
        }
    }

    fn shrink_to(&mut self, new_capacity: isize) {
        let capacity = self.data.capacity();
        if self.back < self.front {
            let shrink = capacity - new_capacity;
            self.data.copy(self.front, self.front-shrink, capacity-self.front);
            self.front -= shrink;
        } else if self.back >= new_capacity {
            let size = self.back - self.front;
            self.data.copy(self.front, 0, size);
            self.front = 0;
            self.back = size;
        }

        self.data.resize(new_capacity);
    }

    fn is_full(&self) -> bool {
        self.increase_index(self.back) == self.front
    }

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
    fn size(&self) -> isize {
        if self.front <= self.back {
            self.back - self.front
        } else {
            self.back + self.data.capacity() - self.front
        }
    }
}


impl<T, P : CapacityPolicy> ListBase for Buffer<T,P> {
    type Elem = T;

    fn get(&self, index:isize) -> Option<&T> {
        if self.front <= self.back {
            if self.front + index >= self.back {
                None
            } else {
                Some(Chunk::get(&self.data, self.front + index))
            }
        } else {
            let capacity = self.data.capacity();
            if self.front + index >= self.back + capacity {
                None
            } else if self.front + index >= capacity {
                Some(Chunk::get(&self.data, self.front + index - capacity))
            } else {
                Some(Chunk::get(&self.data, self.front + index))
            }
        }
    }
}

impl<T, P : CapacityPolicy> ListMutBase for Buffer<T,P> {
    fn get_mut(&mut self, index:isize) -> Option<&mut T> {
        if self.front <= self.back {
            if self.front + index >= self.back {
                None
            } else {
                Some(Chunk::get_mut(&mut self.data, self.front + index))
            }
        } else {
            let capacity = self.data.capacity();
            if self.front + index >= self.back + capacity {
                None
            } else if self.front + index >= capacity {
                Some(Chunk::get_mut(&mut self.data, self.front + index - capacity))
            } else {
                Some(Chunk::get_mut(&mut self.data, self.front + index))
            }
        }
    }
}

impl<T, P : CapacityPolicy> Deque for Buffer<T,P> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.front == self.back
    }

    fn push_front(&mut self, elem: T) {
        if self.is_full() {
            self.grow();
        }

        self.front = self.decrease_index(self.front);
        self.data.write(self.front, elem);
    }

    fn pop_front(&mut self) -> T {
        if self.is_empty() {
            abort!("empty");
        }

        let elem = self.data.read(self.front);
        self.front = self.increase_index(self.front);
        self.shrink();
        elem
    }

    fn push_back(&mut self, elem: T) {
        if self.is_full() {
            self.grow();
        }

        self.data.write(self.back, elem);
        self.back = self.increase_index(self.back);
    }

    fn pop_back(&mut self) -> T {
        if self.is_empty() {
            abort!("empty");
        }

        self.back = self.decrease_index(self.back);
        let elem = self.data.read(self.back);
        self.shrink();
        elem
    }
}

impl<T, P : CapacityPolicy> Drop for Buffer<T,P>{
    fn drop(&mut self){
        if self.back < self.front {
            for i in 0..self.back {
                Chunk::read(&mut self.data, i);
            }

            for i in self.front..self.data.capacity() {
                Chunk::read(&mut self.data, i);
            }
        } else {
            for i in self.front..self.back {
                Chunk::read(&mut self.data, i);
            }
        }
    }
}


#[macro_export]
macro_rules! buffer {
    () => (
        &mut $crate::buffer::Buffer::<_, $crate::capacity::DefaultCapacityPolicy>::new()
    );
}
