use core::cell::RefCell;

pub struct Item<'a> {
    counter: &'a RefCell<usize>,
}

impl<'a> Item<'a> {
    pub fn new(counter: &'a RefCell<usize>) -> Self {
        Item { counter }
    }
}

impl<'a> Drop for Item<'a> {
    fn drop(&mut self) {
        self.counter.replace_with(|x| *x + 1);
    }
}
