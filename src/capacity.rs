pub trait CapacityPolicy {
    fn initial(capacity: isize) -> isize;
    fn grow(capacity: isize) -> isize;
    fn shrink(size: isize, capacity: isize) -> isize;
}

pub struct FixedCapacityPolicy {}

impl CapacityPolicy for FixedCapacityPolicy {
    fn initial(size: isize) -> isize {
        size
    }

    fn grow(capacity: isize) -> isize {
        capacity
    }

    fn shrink(_size: isize, capacity: isize) -> isize {
        capacity
    }
}

pub struct DefaultCapacityPolicy {}

impl CapacityPolicy for DefaultCapacityPolicy {
    fn initial(size: isize) -> isize {
        if size < 10 {
            10
        } else {
            size
        }
    }

    fn grow(capacity: isize) -> isize {
        capacity + (capacity / 2)
    }

    fn shrink(size: isize, capacity: isize) -> isize {
        let new_capacity = if size * 9 / 4 < capacity {
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
