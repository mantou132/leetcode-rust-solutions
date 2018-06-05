use super::collection::Collection;

pub trait Stack : Collection {
    type Elem;

    fn is_empty(&self) -> bool;
    fn push(&mut self, item: Self::Elem);
    fn pop(&mut self) -> Self::Elem;
}
