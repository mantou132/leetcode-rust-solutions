use core::clone::Clone;
use core::cmp::{PartialEq, Eq};
use core::ops::{Deref, Add, Mul};
use core::mem::{size_of, transmute};
use core::ptr::{copy_nonoverlapping, drop_in_place};
use core::slice::from_raw_parts;
use super::libc::{malloc, realloc, free};
use super::traits::InputStream;


struct SharedString {
    counter: *mut usize,
    length: usize,
    s: *const u8,
}

#[cfg(all(target_endian="big", target_pointer_width="64"))]
struct InlineString {
    s: [u8;23],
    length: u8,
}

#[cfg(all(target_endian="little", target_pointer_width="64"))]
struct InlineString {
    length: u8,
    s: [u8;23],
}

#[cfg(all(target_endian="big", target_pointer_width="32"))]
struct InlineString {
    s: [u8;11],
    length: u8,
}

#[cfg(all(target_endian="little", target_pointer_width="32"))]
struct InlineString {
    length: u8,
    s: [u8;11],
}

#[cfg(target_endian="big")]
struct StaticString {
    s: *const u8,
    length: usize,
    padding: usize,
}

#[cfg(target_endian="little")]
struct StaticString {
    padding: usize,
    length: usize,
    s: *const u8,
}

#[cfg(target_pointer_width="64")]
pub struct String {
    _data: [u8;24],
}

#[cfg(target_pointer_width="32")]
pub struct String {
    _data: [u8;12],
}

enum Tag {
    Shared,
    Inline,
    Static,
}

use self::Tag::*;


impl Clone for SharedString {
    fn clone(&self) -> Self {
        unsafe {
            *self.counter += 1;
        }
        SharedString {
            counter: self.counter,
            length: self.length,
            s: self.s,
        }
    }
}

impl Clone for InlineString {
    fn clone(&self) -> Self {
        InlineString {
            length: self.length,
            s: self.s,
        }
    }
}

impl Clone for StaticString {
    fn clone(&self) -> Self {
        StaticString {
            padding: self.padding,
            length: self.length,
            s: self.s,
        }
    }
}

impl Drop for SharedString {
    fn drop(&mut self) {
        unsafe{
            *self.counter -= 1;
            if *self.counter == 0 {
                free(self.counter as *mut _)
            }
        }
    }
}

impl String {
    fn as_inline(&self) -> &InlineString {
        unsafe {
            &*(self as *const _ as *const _)
        }
    }

    fn as_inline_mut(&mut self) -> &mut InlineString {
        unsafe {
            &mut *(self as *mut _ as *mut _)
        }
    }

    fn as_shared(&self) -> &SharedString {
        unsafe {
            &*(self as *const _ as *const _)
        }
    }

    fn as_shared_mut(&mut self) -> &mut SharedString {
        unsafe {
            &mut *(self as *mut _ as *mut _)
        }
    }

    fn as_static(&self) -> &StaticString {
        unsafe {
            &*(self as *const _ as *const _)
        }
    }

    fn as_static_mut(&mut self) -> &mut StaticString {
        unsafe {
            &mut *(self as *mut _ as *mut _)
        }
    }

    fn tag(&self) -> Tag {
        match self.as_inline().length & 0x3 {
            0 => Shared,
            1 => Inline,
            3 => Static,
            _ => abort!("bad string tag"),
        }
    }

    pub fn len(&self) -> usize {
        match self.tag() {
            Shared => self.as_shared().length,
            Inline => (self.as_inline().length >> 3) as usize,
            Static => self.as_static().length,
        }
    }

    fn as_ptr(&self) -> *const u8 {
        match self.tag() {
            Shared => self.as_shared().s,
            Inline => self.as_inline().s.as_ptr(),
            Static => self.as_static().s,
        }
    }

    #[cfg(target_pointer_width="64")]
    fn new_inline() -> InlineString {
        InlineString {
            length: 1,
            s: [0;23],
        }
    }

    #[cfg(target_pointer_width="32")]
    fn new_inline() -> InlineString {
        InlineString {
            length: 1,
            s: [0;11],
        }
    }

    fn new_by_length(length: usize) -> Self {
        if length < size_of::<String>() {
            let mut i = String::new_inline();
            i.length |= (length as u8) << 3;
            unsafe {
                transmute(i)
            }
        } else {
            let ptr =
                unsafe {
                    malloc(size_of::<usize>() + length)
                };

            let s = SharedString {
                counter: ptr as *mut usize,
                length: length,
                s: unsafe { ptr.offset(size_of::<usize>() as isize) }
            };

            unsafe {
                *(s.counter) = 1;
                transmute(s)
            }
        }
    }
}

impl Deref for String {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            from_raw_parts(self.as_ptr(), self.len())
        }
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        match self.tag() {
            Shared =>
                unsafe {
                    transmute(self.as_shared().clone())
                },
            Inline =>
                unsafe {
                    transmute(self.as_inline().clone())
                },
            Static =>
                unsafe {
                    transmute(self.as_static().clone())
                },
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        match self.tag() {
            Shared =>
                unsafe {
                    drop_in_place(self.as_shared_mut() as *mut _);
                },
            Inline =>
                unsafe {
                    drop_in_place(self.as_inline_mut() as *mut _);
                },
            Static =>
                unsafe {
                    drop_in_place(self.as_static_mut() as *mut _);
                },
        }
    }
}

impl PartialEq for String {
    fn eq(&self, other: &String) -> bool {
        let p = self.as_ptr();
        let q = other.as_ptr();

        if self.len() != other.len() {
            false
        } else if p == q {
            true
        } else {
            for i in 0..self.len() {
                if unsafe { *p.offset(i as isize) != *q.offset(i as isize) } {
                    return false;
                }
            }
            true
        }
    }
}

impl Eq for String {
}

impl<'a,'b> Add<&'b String> for &'a String {
    type Output = String;

    fn add(self, other: &'b String) -> String {
        if self.len() == 0 {
            return other.clone();
        } else if other.len() == 0 {
            return self.clone();
        } else if unsafe { self.as_ptr().offset(self.len() as isize) == other.as_ptr() } {
            if let (Shared,Shared) = (self.tag(), other.tag()) {
                let counter = self.as_shared().counter;
                if counter == other.as_shared().counter {
                    unsafe {
                        *(counter) += 1;
                        return transmute(
                            SharedString {
                                counter: counter,
                                length: self.len() + other.len(),
                                s: self.as_ptr(),
                            }
                        );
                    }
                }
            } else if let (Static,Static) = (self.tag(), other.tag()) {
                unsafe {
                    return transmute(
                        StaticString {
                            s: self.as_ptr(),
                            length: self.len() + other.len(),
                            padding: 3,
                        });
                }
            }
        }

        let z = String::new_by_length(self.len() + other.len());
        unsafe {
            copy_nonoverlapping(self.as_ptr(), z.as_ptr() as *mut _, self.len());
            copy_nonoverlapping(other.as_ptr(), z.as_ptr().offset(self.len() as isize) as *mut _, other.len());
        }
        return z;
    }
}

impl Add<String> for String {
    type Output = String;

    fn add(self, other: String) -> String {
        &self + &other
    }
}

impl<'a> Mul<usize> for &'a String {
    type Output = String;

    fn mul(self, other: usize) -> String {
        if other == 1 {
            self.clone()
        } else {
            let length = self.len();
            let z = String::new_by_length(length * other);
            for i in 0..other {
                unsafe {
                    copy_nonoverlapping(self.as_ptr(), z.as_ptr().offset((i * length) as isize) as *mut _, length);
                }
            }
            z
        }
    }
}

impl Mul<usize> for String {
    type Output = String;
    fn mul(self, other: usize) -> String {
        &self * other
    }
}


pub fn string(s: &'static [u8]) -> String {
    let length = s.len();
    if length < size_of::<String>() {
        let mut i = String::new_inline();
        i.length |= (length as u8) << 3;

        unsafe {
            copy_nonoverlapping(s.as_ptr(), i.s.as_mut_ptr(), length);
            return transmute(i);
        }
    } else {
        unsafe {
            transmute(
                StaticString {
                    length: length,
                    s: s.as_ptr(),
                    padding: 3,
                })
        }
    }
}


pub fn read_string_until<Stream: InputStream, Fun: Fn(u8)->bool>(stream: &mut Stream, end: Fun, buffer_size: usize) -> String {
    let mut inline = String::new_inline();
    let inline_size = size_of::<String>()-1;

    for i in 0 .. inline_size {
        match stream.peek() {
            None => {
                inline.length |= (i as u8) << 3;
                unsafe {
                    return transmute(inline);
                }
            },
            Some(&c) if end(c) => {
                inline.length |= (i as u8) << 3;
                unsafe {
                    return transmute(inline);
                }
            },
            Some(&c) =>
                unsafe {
                    *(inline.s.as_mut_ptr().offset(i as isize)) = c;
                },
        }

        stream.consume();
    }

    let ptr =
        unsafe {
            malloc(size_of::<usize>() + buffer_size)
        };

    let mut s = SharedString {
        counter: ptr as *mut usize,
        length: buffer_size,
        s: unsafe { ptr.offset(size_of::<usize>() as isize) },
    };

    unsafe {
        *(s.counter) = 1;
        copy_nonoverlapping(inline.s.as_ptr(), s.s as *mut _, inline_size);
    }

    for i in inline_size .. buffer_size {
        match stream.peek() {
            None => {
                s.length = i;
                break;
            },
            Some(&c) if end(c) => {
                s.length = i;
                break;
            },
            Some(&c) =>
                unsafe {
                    *(s.s.offset(i as isize) as *mut _) = c;
                },
        }

        stream.consume();
    }

    unsafe {
        let ptr = realloc(ptr, size_of::<usize>() + s.length);
        s.counter = ptr as *mut _;
        s.s = ptr.offset(size_of::<usize>() as isize);
        return transmute(s);
    }
}


pub fn read_string<Stream: InputStream>(stream: &mut Stream, buffer_size: usize) -> String {
    read_string_until(stream, |_| {false}, buffer_size)
}


#[macro_export]
macro_rules! str {
    ($s:expr) => {
        &($crate::string::string($s))
    }
}
