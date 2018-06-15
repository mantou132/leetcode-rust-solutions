use super::super::iter::Iterator;
use super::Sink;

pub fn fwrite<'a, S : 'a + Sink, F : FnMut(&'a mut S)>(sink: &'a mut S, mut f: F) {
    f(sink)
}


pub fn join<'a, S : 'a + Sink, Sep : FnMut(&'a mut S), F : FnMut(&'a mut S), I : Iterator<Item=F>>(mut sep: Sep, mut it: I) -> impl FnMut(&'a mut S) {
    move |s: &'a mut S| {
        let iter = &mut it;
        match Iterator::next(iter) {
            None => { return; }
            Some(mut f) => { f(s) }
        }

        for mut f in iter {
            sep(s);
            f(s);
        }
    }
}
