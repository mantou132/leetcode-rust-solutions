use super::super::compat::prelude::*;
use std::error::Error;
use std::fmt;
use std::mem::size_of;
use std::ptr::{read, write};

use super::super::os::{OSError, malloc, realloc, free};

#[derive(Debug)]
pub struct IndexOutOfRange;

impl fmt::Display for IndexOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IndexOutOfRange")
    }
}

impl Error for IndexOutOfRange {
    fn description(&self) -> &str {
        "IndexOutOfRange"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}


pub struct Chunk<T> {
    capacity: usize,
    data: *mut T,
}

impl<T> Chunk<T> {

    pub fn new(capacity: usize) -> Result<Self, OSError> {
        let size = size_of::<T>() * capacity;
        Ok(Chunk {
            capacity: capacity,
            data: malloc(size)? as *mut _,
        })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn as_ptr(&mut self) -> *const T {
        self.data
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }

    pub fn resize(&mut self, capacity: usize) -> Result<(),OSError> {
        let size = size_of::<T>() * capacity;
        self.data = realloc(self.data as *mut _, size)? as *mut _;
        self.capacity = capacity;
        Ok(())
    }

    pub fn get_ptr(&self, index: usize) -> Result<*const T, IndexOutOfRange> {
        if index >= self.capacity {
            Err(IndexOutOfRange)
        } else {
            Ok(unsafe { self.data.offset(index as isize) })
        }
    }

    pub fn get_ptr_mut(&mut self, index: usize) -> Result<*mut T, IndexOutOfRange> {
        if index >= self.capacity {
            Err(IndexOutOfRange)
        } else {
            Ok(unsafe { self.data.offset(index as isize) })
        }
    }

    pub fn read(&mut self, index: usize) -> Result<T, IndexOutOfRange> {
        Ok(unsafe { read(self.get_ptr(index)?) })
    }

    pub fn write(&mut self, index: usize, item: T) -> Result<(), IndexOutOfRange> {
        Ok(unsafe { write(self.get_ptr_mut(index)?, item) })
    }

    pub fn get(&self, index: usize) -> Result<&T, IndexOutOfRange> {
        Ok(unsafe { &*self.get_ptr(index)? })
    }

    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, IndexOutOfRange> {
        Ok(unsafe { &mut *self.get_ptr_mut(index)? })
    }
}


impl<T> Drop for Chunk<T>{
    fn drop(&mut self){
        free(self.data as *mut _)
    }
}
