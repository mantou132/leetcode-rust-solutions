use core::ptr::null_mut;
use super::super::traits::{Allocator, Bounded, Collection, Stack};
use super::super::storage::SystemAllocator;


pub struct ForwardListNode<T> {
    next: *mut ForwardListNode<T>,
    data: T,
}


pub struct ForwardList<T, A : Allocator<Item=ForwardListNode<T>> = SystemAllocator<ForwardListNode<T>>> {
    size: usize,
    top: *mut ForwardListNode<T>,
    allocator: A,
}


impl<T> ForwardList<T, SystemAllocator<ForwardListNode<T>>> {
    pub fn new() -> Self {
        ForwardList {
            size: 0,
            top: null_mut(),
            allocator: SystemAllocator::new(),
        }
    }
}


impl<T, A : Allocator<Item=ForwardListNode<T>>> ForwardList<T,A> {
    pub fn with_allocator(allocator: A) -> Self {
        ForwardList {
            size: 0,
            top: null_mut(),
            allocator: allocator,
        }
    }
}


impl<T, A : Allocator<Item=ForwardListNode<T>>> Collection for ForwardList<T,A> {
    fn size(&self) -> usize {
        self.size
    }
}


impl<T, A : Bounded + Allocator<Item=ForwardListNode<T>>> Bounded for ForwardList<T,A> {
    fn capacity(&self) -> usize {
        Bounded::capacity(&self.allocator)
    }
}


impl<T, A : Allocator<Item=ForwardListNode<T>>> Stack for ForwardList<T,A> {
    type Item = T;

    fn is_empty(&self) -> bool {
        self.top == null_mut()
    }

    fn top(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&(unsafe { &*self.top }.data))
        }
    }

    fn push(&mut self, item: T) {
        let node = ForwardListNode {
            next: self.top,
            data : item,
        };

        self.size += 1;
        self.top = self.allocator.add(node);
    }

    fn pop(&mut self) -> T {
        if self.is_empty() {
            #[cfg(debug_assertions)]
            abort!("empty");
        }

        self.size -= 1;
        let ptr = self.top;
        self.top = unsafe { &*ptr }.next;

        self.allocator.remove(ptr).data
    }
}


impl<T, A : Allocator<Item=ForwardListNode<T>>> Drop for ForwardList<T,A> {
    fn drop(&mut self) {
        while !(self.is_empty()) {
            Stack::pop(self);
        }
    }
}
