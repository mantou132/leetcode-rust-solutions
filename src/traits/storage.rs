pub trait Allocator {
    type Item;

    fn add(&mut self, item: Self::Item) -> *mut Self::Item;
    fn remove(&mut self, item: *mut Self::Item) -> Self::Item;
}


pub trait Bounded {
    fn capacity(&self) -> usize;
}


pub trait Unbounded {
    fn capacity(&self) -> usize;
    fn reserve(&mut self, n: usize);
    fn shrink_to_fit(&mut self);
}
