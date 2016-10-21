pub trait CapacityPolicy {
    fn initial(capacity: usize) -> usize;
}


pub trait FixedCapacityPolicy : CapacityPolicy {
}


pub trait ResizingPolicy : CapacityPolicy {
    fn grow(capacity: usize) -> usize;
    fn shrink(size: usize, capacity: usize) -> usize;
}


pub trait Collection {
    fn size(&self) -> usize;
}


pub trait List : Collection {
    type Item;
    fn get(&self, index: usize) -> &Self::Item;
}


pub trait ListMut : List {
    fn get_mut(&mut self, index: usize) -> &mut Self::Item;

    fn set(&mut self, index: usize, item: Self::Item) {
        *self.get_mut(index) = item;
    }
}


pub trait Stack : Collection {
    type Item;
    fn top(&self) -> Option<&Self::Item>;
    fn is_empty(&self) -> bool;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Self::Item;
}


pub trait Queue : Collection {
    type Item;
    fn is_empty(&self) -> bool;
    fn first(&self) -> Option<&Self::Item>;
    fn last(&self) -> Option<&Self::Item>;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Self::Item;
}


pub trait Deque : Collection {
    type Item;
    fn is_empty(&self) -> bool;
    fn front(&self) -> Option<&Self::Item>;
    fn back(&self) -> Option<&Self::Item>;
    fn push_front(&mut self, item: Self::Item);
    fn pop_front(&mut self) -> Self::Item;
    fn push_back(&mut self, item: Self::Item);
    fn pop_back(&mut self) -> Self::Item;
}

impl<T : Deque> Stack for T {
    type Item = T::Item;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn top(&self) -> Option<&Self::Item> {
        self.back()
    }

    fn push(&mut self, item: Self::Item) {
        self.push_back(item)
    }

    fn pop(&mut self) -> Self::Item {
        self.pop_back()
    }
}


impl<T : Deque> Queue for T {
    type Item = T::Item;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn first(&self) -> Option<&Self::Item> {
        self.front()
    }

    fn last(&self) -> Option<&Self::Item> {
        self.back()
    }

    fn push(&mut self, item: Self::Item) {
        self.push_back(item)
    }

    fn pop(&mut self) -> Self::Item {
        self.pop_front()
    }
}
