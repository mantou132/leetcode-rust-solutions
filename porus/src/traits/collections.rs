pub trait Queue : Collection {
    type Item;
    fn is_empty(&self) -> bool;
    fn first(&self) -> Option<&Self::Item>;
    fn last(&self) -> Option<&Self::Item>;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Self::Item;
}

pub trait Map : Collection {
    type Key;
    type Value;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}
