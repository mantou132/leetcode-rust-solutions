pub struct Range {
    start: Option<isize>,
    stop: Option<isize>,
    step: isize,
}

impl Range {
    pub fn new(start: Option<isize>, stop: Option<isize>, step: isize) -> Self {
        if step == 0 {
            abort!("step cannot be zero")
        }

        Range {
            start: start,
            stop: stop,
            step: step,
        }
    }

    pub fn start(&self, size: isize) -> isize {
        match self.start {
            Some(x) if x < 0 => size + x,
            Some(x) => x,
            None if self.step > 0 => 0,
            None if self.step < 0 => size - 1,
            None => unreachable!(),
        }
    }

    pub fn stop(&self, size: isize) -> isize {
        match self.stop {
            Some(x) if x < 0 => size + x,
            Some(x) => x,
            None if self.step > 0 => size,
            None if self.step < 0 => -1,
            None => unreachable!(),
        }
    }

    pub fn step(&self) -> isize {
        self.step
    }
}

#[macro_export]
macro_rules! range {
    () => ( &$crate::range::Range::new(None, None, 1) );
    ($stop:expr) => ( &$crate::range::Range::new(Some(0), Some($stop), 1) );
    ($start:expr, $stop:expr) => ( &$crate::range::Range::new(Some($start), Some($stop), 1) );
    ($start:expr, $stop:expr, $step:expr) => ( &$crate::range::Range::new(Some($start), Some($stop), $step) );
    ($start:expr, , $step:expr) => ( &$crate::range::Range::new(Some($start), None, $step) );
    (, $stop:expr, $step:expr) => ( &$crate::range::Range::new(None, Some($stop), $step) );
    (, , $step:expr) => ( &$crate::range::Range::new(None, None, $step) );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_range_start() {
        assert!(range!().start(2) == 0);
        assert!(range!(3).start(2) == 0);
        assert!(range!(1,3).start(2) == 1);
        assert!(range!(-5,-1).start(10) == 5);
        assert!(range!(-5,-1,2).start(10) == 5);
        assert!(range!(-5,,2).start(10) == 5);
        assert!(range!(,-1,2).start(10) == 0);
        assert!(range!(,-1,-1).start(10) == 9);
        assert!(range!(,,-1).start(10) == 9);
    }

    #[test]
    fn test_range_stop() {
        assert!(range!().stop(2) == 2);
        assert!(range!(3).stop(2) == 3);
        assert!(range!(1,3).stop(2) == 3);
        assert!(range!(-5,-1).stop(10) == 9);
        assert!(range!(-5,-1,2).stop(10) == 9);
        assert!(range!(-5,,2).stop(10) == 10);
        assert!(range!(,-1,2).stop(10) == 9);
        assert!(range!(,-1,-1).stop(10) == 9);
        assert!(range!(,,-1).stop(10) == -1);
    }

    #[test]
    fn test_range_step() {
        assert!(range!().step() == 1);
        assert!(range!(1).step() == 1);
        assert!(range!(0,1,2).step() == 2);
        assert!(range!(0,,2).step() == 2);
        assert!(range!(,1,2).step() == 2);
        assert!(range!(,,2).step() == 2);
    }
}
