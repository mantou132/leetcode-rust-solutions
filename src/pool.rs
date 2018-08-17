pub trait Handle: Copy + Eq + Default {}

pub trait Pool<T> {
    type Handle: Handle;

    fn get(&self, handle: Self::Handle) -> &T;
    fn get_mut(&mut self, handle: Self::Handle) -> &mut T;
    fn add(&mut self, item: T) -> Self::Handle;
    fn remove(&mut self, item: Self::Handle) -> T;
}
