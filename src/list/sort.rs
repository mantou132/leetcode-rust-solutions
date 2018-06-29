use core::mem;
use super::{ListMut, get_mut};
use super::super::collection::Collection;


fn swap<L: ListMut>(list: &mut L, i: isize, j: isize) {
    let mut t = unsafe { mem::uninitialized() };
    mem::swap(&mut t, get_mut(list, i).unwrap());
    mem::swap(&mut t, get_mut(list, j).unwrap());
    mem::swap(&mut t, get_mut(list, i).unwrap());
    mem::forget(t);
}

pub fn bubble<L: ListMut + Collection>(list: &mut L) -> usize where L::Elem : Ord {
    let mut count = 0;
    let size = Collection::size(list);
    if size >= 2 {
        let mut i = size - 1;
        while i > 0 {
            if list[i] < list[i-1] {
                swap(list, i, i-1);
                count += 1;
            }

            i -= 1;
        }
    }
    count
}

pub fn bubble_sorted<L: ListMut + Collection>(list: &mut L) -> usize where L::Elem : Ord {
    let mut count = 0;
    let size = Collection::size(list);
    if size >= 2 {
        let mut i = size - 1;
        while (i > 0) && (list[i] < list[i-1]) {
            swap(list, i, i-1);
            count += 1;
            i -= 1;
        }
    }
    count
}

pub fn insertion_sort<L: ListMut + Collection>(list: &mut L) -> usize where L::Elem : Ord {
    let mut count = 0;
    let size = Collection::size(list);
    for i in 2..size+1 {
        count += bubble_sorted(slice_mut!(list, [0, i]));
    }
    count
}

pub fn bubble_sort<L: ListMut + Collection>(list: &mut L) -> usize where L::Elem : Ord {
    let mut count = 0;
    let size = Collection::size(list);
    for i in 0..size-1 {
        count += bubble(slice_mut!(list, [i, size]));
    }
    count
}
