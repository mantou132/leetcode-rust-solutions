use std::mem::size_of;
use std::marker::PhantomData;
use super::raw::RawSystemChunk;


pub struct SystemChunk<T> {
    data: RawSystemChunk,
    _marker: PhantomData<T>,
}

impl<T> SystemChunk<T> {

    fn size() -> isize {
        size_of::<T>() as _
    }

    pub fn new(capacity: isize) -> Self {
        let size = Self::size() * capacity;
        SystemChunk {
            data: RawSystemChunk::new(size),
            _marker: PhantomData,
        }
    }

    pub fn capacity(&self) -> isize {
        self.data.capacity() / Self::size()
    }

    pub fn resize(&mut self, capacity: isize) {
        self.data.resize(Self::size() * capacity);
    }

    fn check_index(&self, mut index: isize) -> Option<isize> {
        index = Self::size() * index;
        if (0 <= index) && (index < self.data.capacity()) {
            Some(index)
        } else {
            None
        }
    }

    pub fn read(&mut self, index: isize) -> Option<T> {
        self.check_index(index).map(|i| self.data.read(i))
    }

    pub fn write(&mut self, index: isize, item: T) {
        if let Some(i) = self.check_index(index) {
            self.data.write(i, item);
        }
    }

    pub fn get(&self, index: isize) -> Option<&T> {
        self.check_index(index).map(|i| self.data.get(i))
    }

    pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        self.check_index(index).map(move |i| self.data.get_mut(i))
    }
}
