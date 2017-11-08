use super::super::compat::prelude::*;
use super::{PeekableSource, ScanError};
use super::num::{read_unsigned, read_signed};


pub trait ReadArg<T> {
    fn read<S: PeekableSource<Item=T>>(source: &mut S, Self) -> Result<(), ScanError<S::Error>>;
}

pub fn read<T, R: ReadArg<T>, S: PeekableSource<Item=T>>(source: &mut S, r: R) -> Result<(), ScanError<S::Error>> {
    ReadArg::read(source, r)
}


impl<'a> ReadArg<u8> for &'a mut u8 {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_unsigned(source, x)
    }
}

impl<'a> ReadArg<u8> for &'a mut u16 {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_unsigned(source, x)
    }
}

impl<'a> ReadArg<u8> for &'a mut u32 {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_unsigned(source, x)
    }
}

impl<'a> ReadArg<u8> for &'a mut u64 {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_unsigned(source, x)
    }
}

impl<'a> ReadArg<u8> for &'a mut usize {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_unsigned(source, x)
    }
}

impl<'a> ReadArg<u8> for &'a mut i8 {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_signed(source, x)
    }
}

impl<'a> ReadArg<u8> for &'a mut i16 {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_signed(source, x)
    }
}

impl<'a> ReadArg<u8> for &'a mut i32 {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_signed(source, x)
    }
}

impl<'a> ReadArg<u8> for &'a mut i64 {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_signed(source, x)
    }
}

impl<'a> ReadArg<u8> for &'a mut isize {
    fn read<S: PeekableSource<Item=u8>>(source: &mut S, x: Self) -> Result<(), ScanError<S::Error>> {
        read_signed(source, x)
    }
}
