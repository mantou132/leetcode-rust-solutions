use std::ptr;

pub fn copy<T>(p: *mut T, src: isize, dst: isize, count: isize) {
    unsafe {
        ptr::copy(p.offset(src), p.offset(dst), count as usize);
    }
}

pub fn read<T>(p: *mut T, index: isize) -> T {
    unsafe {
        ptr::read(p.offset(index))
    }
}

pub fn write<T>(p: *mut T, index: isize, item: T) {
    unsafe {
        ptr::write(p.offset(index), item)
    }
}

pub fn get<'a, T>(p: *mut T, index: isize) -> &'a T {
    unsafe { &*p.offset(index) }
}

pub fn get_mut<'a, T>(p: *mut T, index: isize) -> &'a mut T {
    unsafe { &mut *p.offset(index) }
}
