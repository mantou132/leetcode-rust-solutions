use core::mem;
use super::{List, ListMut, get_mut, iter};
use super::slice::ListMutView;
use super::super::collection::Collection;


fn swap<L: ListMut>(list: &mut L, i: isize, j: isize) {
    if i == j {
        return;
    }

    let mut t = unsafe { mem::uninitialized() };
    mem::swap(&mut t, get_mut(list, i).unwrap());
    mem::swap(&mut t, get_mut(list, j).unwrap());
    mem::swap(&mut t, get_mut(list, i).unwrap());
    mem::forget(t);
}

pub fn is_stable_sort<E, L: List<Elem=E> + Collection, F: Fn(&E, &E) -> bool, I: List<Elem=isize>>(list: &L, lt: &F, index: &I) -> bool {
    for i in 0..(Collection::size(list)-1) {
        if !lt(&list[index[i]], &list[index[i+1]]) && !(index[i] < index[i+1]) {
            return false;
        }
    }
    return true;
}

pub fn bubble<E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool>(list: &mut L, lt: &F) -> usize {
    let mut count = 0;
    let size = Collection::size(list);
    if size >= 2 {
        let mut i = size - 1;
        while i > 0 {
            if lt(&list[i], &list[i-1]) {
                swap(list, i, i-1);
                count += 1;
            }

            i -= 1;
        }
    }
    count
}

pub fn bubble_sort<E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool>(list: &mut L, lt: &F) -> usize {
    let mut count = 0;
    let size = Collection::size(list);
    for i in 0..size-1 {
        count += bubble(slice_mut!(list, [i, size]), lt);
    }
    count
}

pub fn bubble_sorted<E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool>(list: &mut L, lt: &F) -> usize {
    let mut count = 0;
    let size = Collection::size(list);
    let mut i = size - 1;
    while (i > 0) && lt(&list[i], &list[i-1]) {
        swap(list, i, i-1);
        count += 1;
        i -= 1;
    }
    count
}

fn insertion_sort_g<E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool>(list: &mut L, lt: &F, g: isize) -> usize {
    let mut count = 0;
    let size = Collection::size(list);

    for i in g..size {
        let mut j = i;
        while ((j - g) >= 0) && lt(&list[j], &list[j-g]) {
            swap(list, j, j-g);
            count += 1;
            j -= g;
        }
    }

    count
}

pub fn insertion_sort<E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool>(list: &mut L, lt: &F) -> usize {
    // let mut count = 0;
    // let size = Collection::size(list);
    // for i in 2..size+1 {
    //     count += bubble_sorted(slice_mut!(list, [0, i]), lt);
    // }
    // count
    insertion_sort_g(list, lt, 1)
}

pub fn shell_sort<E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool, G : List<Elem=isize> + Collection>(list: &mut L, lt: &F, gaps: &G) -> usize {
    let mut count = 0;
    for g in iter(gaps) {
        // for i in 0..g {
        //     count += insertion_sort(slice_mut!(list, [i,,g]), lt);
        // }
        count += insertion_sort_g(list, lt, g);
    }
    count
}

pub fn selection_sort<E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool>(list: &mut L, lt: &F) -> usize {
    let mut count = 0;
    let size = Collection::size(list);
    for i in 0..size {
        let mut min = i;
        for j in i+1..size {
            if lt(&list[j], &list[min]) {
                min = j;
            }
        }

        if min != i {
            swap(list, i, min);
            count += 1;
        }
    }
    count
}

pub fn partition<E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool>(list: &mut L, lt: &F) -> isize {
    let size = Collection::size(list);
    let mut i = 0;
    for j in 0..size-1 {
        if lt(&list[j], &list[size-1]) {
            swap(list, j, i);
            i += 1;
        }
    }

    swap(list, i, size-1);
    i
}


fn quick_sort_aux<'a, 'b: 'a, E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool>(list: &'b mut ListMutView<'a, L>, lt: &F) {
    let size = Collection::size(list);
    if size < 2 {
        return;
    }

    let p = partition(list, lt);
    quick_sort_aux::<E,L,F>(slice_mut!(list, [,p]), lt);
    quick_sort_aux::<E,L,F>(slice_mut!(list, [p+1,]), lt);
}

pub fn quick_sort<E, L: ListMut<Elem=E> + Collection, F: Fn(&E, &E) -> bool>(list: &mut L, lt: &F) {
    quick_sort_aux(slice_mut!(list, [0,]), lt);
}
