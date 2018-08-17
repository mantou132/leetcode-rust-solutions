use core::ptr;

pub unsafe fn copy<T>(p: *mut T, src: isize, dst: isize, count: isize) {
    ptr::copy(p.offset(src), p.offset(dst), count as usize);
}

pub unsafe fn read<T>(p: *mut T, index: isize) -> T {
    ptr::read(p.offset(index))
}

pub unsafe fn write<T>(p: *mut T, index: isize, item: T) {
    ptr::write(p.offset(index), item)
}

pub unsafe fn get<'a, T>(p: *mut T, index: isize) -> &'a T {
    &*p.offset(index)
}

pub unsafe fn get_mut<'a, T>(p: *mut T, index: isize) -> &'a mut T {
    &mut *p.offset(index)
}
