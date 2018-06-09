pub trait Stack {
    type Elem;

    fn is_empty(&self) -> bool;
    fn push(&mut self, elem: Self::Elem);
    fn pop(&mut self) -> Self::Elem;
}
