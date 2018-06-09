pub trait Deque {
    type Elem;

    fn is_empty(&self) -> bool;
    fn push_front(&mut self, elem: Self::Elem);
    fn pop_front(&mut self) -> Self::Elem;
    fn push_back(&mut self, elem: Self::Elem);
    fn pop_back(&mut self) -> Self::Elem;
}
