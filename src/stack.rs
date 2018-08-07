pub trait Stack {
    type Elem;

    fn is_empty(&self) -> bool;
    fn push(&mut self, elem: Self::Elem);
    fn pop(&mut self) -> Self::Elem;
}

pub fn is_empty<T: Stack>(s: &T) -> bool {
    Stack::is_empty(s)
}

pub fn push<T: Stack>(s: &mut T, elem: T::Elem) {
    Stack::push(s, elem)
}

pub fn pop<T: Stack>(s: &mut T) -> T::Elem {
    Stack::pop(s)
}
