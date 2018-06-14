use super::Iter;

pub struct Peekable<I: Iter> {
    iter: I,
    peeked: Option<Option<I::Item>>,
}

impl<I : Iter> Peekable<I> {
    pub const fn new(it: I) -> Self {
        Peekable {
            iter: it,
            peeked: None,
        }
    }

    pub fn peek<'a>(&'a mut self) -> Option<&'a I::Item> {
        if let None = self.peeked {
            self.consume();
        }

        if let Some(ref x) = self.peeked {
            return x.as_ref();
        }

        unreachable!();
    }

    pub fn consume(&mut self) {
        self.peeked = Some(Iter::next(&mut self.iter));
    }
}

impl<I : Iter> Iter for Peekable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.peeked {
            self.consume();
        }

        let result = self.peeked.take();
        self.consume();

        if let Some(x) = result {
            return x;
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::super::Iter;
    use super::super::into_iter;

    #[test]
    fn test_peek() {
        let it = &mut into_iter(&[1,2,3] as &_).peek();
        assert!(Some(&1) == it.peek());
        it.consume();
        assert!(Some(&2) == it.peek());
        it.consume();
        assert!(Some(&3) == it.peek());
        it.consume();
        assert!(None == it.peek());
    }
}
