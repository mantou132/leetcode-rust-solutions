use super::super::traits;

pub struct InputStream<'a> {
    s: &'a [u8],
}

impl<'a> traits::PeekingIterator for InputStream<'a> {
    type Item = u8;

    fn peek<'b>(&'b mut self) -> Option<&'b Self::Item> {
        self.s.first()
    }

    fn consume(&mut self) {
        match self.s.split_first() {
            Some((_,s)) => self.s = s,
            _ => {},
        }
    }
}

impl<'a> InputStream<'a> {
    pub fn new(s: &'a [u8]) -> InputStream<'a> {
        InputStream {
            s: s,
        }
    }
}

pub struct OutputStream<'a> {
    offset: usize,
    s: &'a mut [u8],
}

impl<'a> traits::OutputStream for OutputStream<'a> {
    fn write(&mut self, c: u8) {
        if self.offset == self.s.len() {
            abort!("buffer overflow");
        }

        self.s[self.offset] = c;
        self.offset += 1;
    }
}

impl<'a> OutputStream<'a> {
    pub fn new(s: &'a mut[u8]) -> OutputStream<'a> {
        OutputStream {
            offset: 0,
            s: s,
        }
    }
}
