use super::collection::Collection;

pub trait Deque : Collection {
    type Elem;

    fn is_empty(&self) -> bool;
    fn push_front(&mut self, item: Self::Elem);
    fn pop_front(&mut self) -> Self::Elem;
    fn push_back(&mut self, item: Self::Elem);
    fn pop_back(&mut self) -> Self::Elem;
}
