use super::super::traits::{CapacityPolicy, FixedCapacityPolicy, ResizingPolicy};


pub struct FixedCapacity {
}


impl CapacityPolicy for FixedCapacity {
    fn initial(capacity: usize) -> usize {
        capacity
    }
}


impl FixedCapacityPolicy for FixedCapacity {
}


pub struct DefaultResizingPolicy {
}


impl CapacityPolicy for DefaultResizingPolicy {
    fn initial(size: usize) -> usize {
        if size < 10 {
            10
        } else {
            size
        }
    }
}


impl ResizingPolicy for DefaultResizingPolicy {

    fn grow(capacity: usize) -> usize {
        capacity + (capacity / 2)
    }

    fn shrink(size: usize, capacity: usize) -> usize {
        let new_capacity =
            if size * 9 / 4 < capacity {
                size * 3 / 2
            } else {
                capacity
            };
        if new_capacity < 10 {
            10
        } else {
            new_capacity
        }
    }
}
