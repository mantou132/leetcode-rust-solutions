use std::ops::{Add, Mul, Div, Rem, Neg};
use super::scanf::{Converter, CharPattern, SignedPattern, UnsignedPattern};
use super::printf::IntField;

pub trait FromChar {
    fn from_char(u8) -> Self;
}

pub trait ToChar {
    fn to_char(self) -> u8;
}

pub trait Unsigned : Sized + Copy + Add<Self,Output=Self> + Mul<Self,Output=Self> + Div<Self,Output=Self> + Rem<Self,Output=Self> + Ord + FromChar + ToChar {
}

pub trait Signed : Sized + Copy + Add<Self,Output=Self> + Mul<Self,Output=Self> + Neg<Output=Self> + Div<Self,Output=Self> + Rem<Self,Output=Self> + Ord + FromChar + ToChar {
}


impl FromChar for u8 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for u16 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for u32 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for u64 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for usize {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for i8 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for i16 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for i32 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for i64 {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}

impl FromChar for isize {
    fn from_char(x : u8) -> Self {
        x as Self
    }
}


impl ToChar for u8 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for u16 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for u32 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for u64 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for usize {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for i8 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for i16 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for i32 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for i64 {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl ToChar for isize {
    fn to_char(self) -> u8 {
        self as u8
    }
}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for usize {}
impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for isize {}


fn from_digit<T : FromChar>(c: u8) -> T {
    FromChar::from_char(
        match c {
            b'0' ... b'9' => { c - b'0' },
            b'A' ... b'Z' => { c - b'A' + 10 },
            b'a' ... b'z' => { c - b'a' + 10 },
            _ => { abort!() },
        })
}


fn to_digit<T : ToChar>(x: T) -> u8 {
    let c = ToChar::to_char(x);
    match c {
        0 ... 9 => { b'0' + c },
        10 ... 35 => { b'A' + c - 10 },
        _ => { abort!() },
    }
}


pub struct CharConverter<T> {
    data: T,
}

impl<'a, T: FromChar> Converter for CharConverter<&'a mut T> {
    fn write(&mut self, c: u8) {
        *self.data = FromChar::from_char(c);
    }
}

impl<'a, T: FromChar> CharPattern for &'a mut T {
    type Converter = CharConverter<Self>;

    fn converter(self) -> Self::Converter {
        CharConverter {
            data: self,
        }
    }
}


pub struct UnsignedConverter<'a, T: 'a + Unsigned> {
    base: T,
    data: &'a mut T,
}

impl<'a, T: 'a + Unsigned> Converter for UnsignedConverter<'a, T> {
    fn write(&mut self, c: u8) {
        *self.data = *self.data * self.base + from_digit(c);
    }
}

impl<'a, T: 'a + Unsigned> UnsignedPattern for &'a mut T {
    type Converter = UnsignedConverter<'a, T>;

    fn converter(self, base: u8) -> Self::Converter {
        *self = FromChar::from_char(0);
        UnsignedConverter {
            base: FromChar::from_char(base),
            data: self,
        }
    }
}

pub struct SignedConverter<'a, T: 'a + Signed> {
    base: T,
    sign: T,
    data: &'a mut T,
}

impl<'a, T: 'a + Signed> Converter for SignedConverter<'a, T> {
    fn write(&mut self, c: u8) {
        if let b'-' = c {
            self.sign = -self.sign;
        } else {
            *self.data = *self.data * self.base + from_digit(c);
        }
    }
}

impl<'a, T: 'a + Signed> SignedPattern for &'a mut T {
    type Converter = SignedConverter<'a, T>;

    fn converter(self, base: u8) -> Self::Converter {
        *self = FromChar::from_char(0);
        SignedConverter {
            base: FromChar::from_char(base),
            sign: FromChar::from_char(1),
            data: self,
        }
    }
}

impl<'a, T: 'a + Signed> Drop for SignedConverter<'a, T> {
    fn drop(&mut self) {
        *self.data = *self.data * self.sign;
    }
}

pub struct IntBuffer {
    offset: u8,
    buf: [u8;64],
}

impl AsRef<[u8]> for IntBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            self.buf.get_unchecked(self.offset as _..64)
        }
    }
}

impl IntBuffer {
    fn new_unsigned<T: Unsigned>(mut n: T, x: u8) -> Self {
        let base = FromChar::from_char(x);
        let zero = FromChar::from_char(0u8);
        let mut buf = IntBuffer {
            offset: if n == zero { 63 } else { 64 },
            buf: [b'0';64],
        };

        while n != zero {
            buf.offset -= 1;
            * unsafe { buf.buf.get_unchecked_mut(buf.offset as usize) } = to_digit(n % base);
            n = n / base;
        }

        buf
    }

    fn new_signed<T: Signed>(mut n: T, x: u8) -> Self {
        let base = FromChar::from_char(x);
        let zero = FromChar::from_char(0u8);
        let neg = n < zero;
        let mut buf = IntBuffer {
            offset: if n == zero { 63 } else { 64 },
            buf: [if neg { b'-' } else { b'0' };64],
        };

        while n != zero {
            buf.offset -= 1;
            let rem = n % base;
            * unsafe { buf.buf.get_unchecked_mut(buf.offset as usize) } = to_digit(if neg { -rem } else { rem });
            n = n / base;
        }

        if neg {
            buf.offset -= 1;
        }

        buf
    }
}

impl IntField for u8 {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_unsigned(self, base)
    }
}

impl IntField for u16 {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_unsigned(self, base)
    }
}

impl IntField for u32 {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_unsigned(self, base)
    }
}

impl IntField for u64 {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_unsigned(self, base)
    }
}

impl IntField for usize {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_unsigned(self, base)
    }
}

impl IntField for i8 {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_signed(self, base)
    }
}

impl IntField for i16 {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_signed(self, base)
    }
}

impl IntField for i32 {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_signed(self, base)
    }
}

impl IntField for i64 {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_signed(self, base)
    }
}

impl IntField for isize {
    type Converter = IntBuffer;

    fn converter(self, base: u8) -> IntBuffer {
        IntBuffer::new_signed(self, base)
    }
}
