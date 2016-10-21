use std::cell::RefCell;
use std::rc::Rc;

pub struct Counter {
    count: usize,
}

pub struct Item {
    counter: Rc<RefCell<Counter>>,
}

impl Counter {
    pub fn new() -> Rc<RefCell<Counter>> {
        Rc::new(RefCell::new(Counter { count: 0 }))
    }

    pub fn incr(&mut self) {
        self.count += 1;
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl Item {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Item {
        Item {
            counter: counter,
        }
    }
}


impl Drop for Item {
    fn drop(&mut self) {
        self.counter.borrow_mut().incr();
    }
}
