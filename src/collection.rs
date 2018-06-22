pub trait Collection {
    fn size(&self) -> isize;
}

pub fn size<T: Collection>(c: &T) -> isize {
    Collection::size(c)
}
