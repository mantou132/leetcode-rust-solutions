use porus::traits;

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

impl<'a> traits::InputStream for InputStream<'a> {
}

impl<'a> InputStream<'a> {
    pub fn new(s: &'a [u8]) -> Self {
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
    fn write_char(&mut self, c: u8) {
        if self.offset == self.s.len() {
            abort!("buffer overflow");
        }

        self.s[self.offset] = c;
        self.offset += 1;
    }
}

impl<'a> OutputStream<'a> {
    pub fn new(s: &'a mut[u8]) -> Self {
        OutputStream {
            offset: 0,
            s: s,
        }
    }
}
