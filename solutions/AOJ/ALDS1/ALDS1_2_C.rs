#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

#[derive(Default, Clone, Copy)]
struct Card(u8, u8);

fn read_card() -> Card {
    let mut suit = 0u8;
    read!(Char(&mut suit));
    Card(suit, read!())
}

fn stable(b: bool) -> &'static str {
    if b {
        "Stable"
    } else {
        "Not stable"
    }
}

fn solve() {
    let n: isize = read!();
    let a = &mut Array::<Card>::new_from_iter((0..n).map(|_| read_card()));

    let bi = &mut Array::<isize>::new_from_iter(0..n);
    list::bubble_sort(bi, &|i, j| a[*i].1 < a[*j].1);
    writelnf!(
        "{}",
        join(
            f!(" "),
            list::iter(bi).map(|i| f!("{:c}{:d}", a[i].0, a[i].1))
        )
    );
    writelnf!(
        "{:s}",
        stable(list::is_stable_sort(a, &|x, y| x.1 < y.1, bi))
    );

    let si = &mut Array::<isize>::new_from_iter(0..n);
    list::selection_sort(si, &|i, j| a[*i].1 < a[*j].1);
    writelnf!(
        "{}",
        join(
            f!(" "),
            list::iter(si).map(|i| f!("{:c}{:d}", a[i].0, a[i].1))
        )
    );
    writelnf!(
        "{:s}",
        stable(list::is_stable_sort(a, &|x, y| x.1 < y.1, si))
    );
}
