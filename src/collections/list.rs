use core::ptr::null_mut;
use super::super::traits::{Allocator, Bounded, Collection, Deque, Stack};
use super::super::storage::SystemAllocator;


pub struct LinkedListHeader {
    prev: *mut LinkedListHeader,
    next: *mut LinkedListHeader,
}


pub struct LinkedListNode<T> {
    header: LinkedListHeader,
    data: T,
}


pub struct LinkedList<T, A : Allocator<Item=LinkedListNode<T>> = SystemAllocator<LinkedListNode<T>>> {
    size: usize,
    front: *mut LinkedListNode<T>,
    back: *mut LinkedListNode<T>,
    allocator: A,
}


impl<T> LinkedList<T, SystemAllocator<LinkedListNode<T>>> {
    pub fn new() -> Self {
        LinkedList {
            size: 0,
            front: null_mut(),
            back: null_mut(),
            allocator: SystemAllocator::new(),
        }
    }
}


impl<T, A : Allocator<Item=LinkedListNode<T>>> LinkedList<T,A> {
    pub fn new_with_allocator(allocator: A) -> Self {
        LinkedList {
            size: 0,
            front: null_mut(),
            back: null_mut(),
            allocator: allocator,
        }
    }
}


impl<T, A : Allocator<Item=LinkedListNode<T>>> Collection for LinkedList<T,A> {
    fn size(&self) -> usize {
        self.size
    }
}


impl<T, A : Bounded + Allocator<Item=LinkedListNode<T>>> Bounded for LinkedList<T,A> {
    fn capacity(&self) -> usize {
        Bounded::capacity(&self.allocator)
    }
}


impl<T, A : Allocator<Item=LinkedListNode<T>>> Deque for LinkedList<T,A> {
    type Item = T;

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn front(&self) -> Option<&T> {
        if Deque::is_empty(self) {
            None
        } else {
            Some(&(unsafe { &*self.front }.data))
        }
    }

    fn back(&self) -> Option<&T> {
        if Deque::is_empty(self) {
            None
        } else {
            Some(&(unsafe { &*self.back }.data))
        }
    }

    fn push_front(&mut self, item: T) {
        let node = LinkedListNode {
            header: LinkedListHeader {
                prev: null_mut(),
                next: self.front as *mut LinkedListHeader,
            },
            data: item,
        };

        let ptr = self.allocator.add(node);
        self.size += 1;

        if self.front != null_mut() {
            unsafe { &mut *self.front }.header.prev = ptr as *mut LinkedListHeader;
        } else {
            self.back = ptr;
        }

        self.front = ptr;
    }

    fn pop_front(&mut self) -> T {
        if Deque::is_empty(self) {
            #[cfg(debug_assertions)]
            abort!("empty");
        }

        self.size -= 1;
        let ptr = self.front;

        self.front = unsafe { &*ptr }.header.next as *mut LinkedListNode<T>;

        if self.front != null_mut() {
            unsafe { &mut *self.front }.header.prev = null_mut();
        } else {
            self.back = null_mut();
        }

        self.allocator.remove(ptr).data
    }

    fn push_back(&mut self, item: T) {
        let node = LinkedListNode {
            header: LinkedListHeader {
                prev: self.back as *mut LinkedListHeader,
                next: null_mut(),
            },
            data: item,
        };

        let ptr = self.allocator.add(node);
        self.size += 1;

        if self.back != null_mut() {
            unsafe { &mut *self.back }.header.next = ptr as *mut LinkedListHeader;
        } else {
            self.front = ptr;
        }

        self.back = ptr;
    }

    fn pop_back(&mut self) -> T {
        if Deque::is_empty(self) {
            #[cfg(debug_assertions)]
            abort!("empty");
        }

        self.size -= 1;
        let ptr = self.back;

        self.back = unsafe { &*ptr }.header.prev as *mut LinkedListNode<T>;

        if self.back != null_mut() {
            unsafe { &mut *self.back }.header.next = null_mut();
        } else {
            self.back = null_mut();
        }

        self.allocator.remove(ptr).data
    }

}


impl<T, A : Allocator<Item=LinkedListNode<T>>> Drop for LinkedList<T,A> {
    fn drop(&mut self) {
        while !(Deque::is_empty(self)) {
            Stack::pop(self);
        }
    }
}
