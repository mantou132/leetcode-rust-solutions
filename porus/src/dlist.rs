use std::marker::PhantomData;
use super::pool::{Pool, Handle};
use super::os::{Handle as OSHandle, OSAllocator};
use super::deque::Deque;

pub struct Link<H : Handle> {
    prev: H,
    next: H,
}

pub struct Node<H : Handle, T> {
    link: Link<H>,
    data: T,
}

pub struct DoublyLinkedList<T, H : Handle, P : Pool<Node<H, T>, Handle=H> = OSAllocator> {
    pool: P,
    sentinel: Link<H>,
    _data: std::marker::PhantomData<T>,
}

impl <T> DoublyLinkedList<T, OSHandle, OSAllocator> {
    pub fn new() -> Self {
        DoublyLinkedList {
            pool: OSAllocator {},
            sentinel: Link {
                prev : Default::default(),
                next : Default::default(),
            },
            _data: PhantomData
        }
    }
}

impl <T, H : Handle, P : Pool<Node<H, T>, Handle=H>> DoublyLinkedList<T,H,P> {

    pub fn front(&self) -> H {
        self.sentinel.next
    }

    pub fn back(&self) -> H {
        self.sentinel.prev
    }

    fn add_node(&mut self, data: T) -> H {
        let node =
            Node {
                link: Link {
                    prev : Default::default(),
                    next : Default::default(),
                },
                data : data,
            };
        Pool::add(&mut self.pool, node)
    }

    fn get_link(&self, handle: H) -> &Link<H> {
        if handle == <H as Default>::default() {
            &self.sentinel
        } else {
            &Pool::get(&self.pool, handle).link
        }
    }

    fn get_node_mut(&mut self, handle: H) -> &mut Link<H> {
        if handle == <H as Default>::default() {
            &mut self.sentinel
        } else {
            &mut Pool::get_mut(&mut self.pool, handle).link
        }
    }

    pub fn prev(&self, handle: H) -> H {
        self.get_link(handle).prev
    }

    pub fn next(&self, handle: H) -> H {
        self.get_link(handle).next
    }

    pub fn get(&self, handle: H) -> &T {
        &Pool::get(&self.pool, handle).data
    }

    pub fn get_mut(&mut self, handle: H) -> &mut T {
        &mut Pool::get_mut(&mut self.pool, handle).data
    }

    pub fn insert_before(&mut self, data: T, reference: H) -> H {
        let new = self.add_node(data);
        let prev = self.prev(reference);
        self.get_node_mut(reference).prev = new;
        self.get_node_mut(new).next = reference;
        self.get_node_mut(new).prev = prev;
        self.get_node_mut(prev).next = new;
        new
    }

    pub fn insert_after(&mut self, data: T, reference: H) -> H {
        let new = self.add_node(data);
        let next = self.next(reference);
        self.get_node_mut(reference).next = new;
        self.get_node_mut(new).prev = reference;
        self.get_node_mut(new).next = next;
        self.get_node_mut(next).prev = new;
        new
    }

    pub fn remove(&mut self, handle: H) -> T {
        let prev = self.prev(handle);
        let next = self.next(handle);
        self.get_node_mut(prev).next = next;
        self.get_node_mut(next).prev = prev;
        Pool::remove(&mut self.pool, handle).data
    }
}

impl <T, H : Handle, P : Pool<Node<H, T>, Handle=H>> Deque for DoublyLinkedList<T,H,P> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.front() == Default::default()
    }

    fn push_front(&mut self, elem: T) {
        let front = self.front();
        self.insert_before(elem, front);
    }

    fn pop_front(&mut self) -> T {
        let front = self.front();
        self.remove(front)
    }

    fn push_back(&mut self, elem: T) {
        let back = self.back();
        self.insert_after(elem, back);
    }

    fn pop_back(&mut self) -> T {
        let back = self.back();
        self.remove(back)
    } 
}
