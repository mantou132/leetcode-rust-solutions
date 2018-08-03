#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

use porus::io::{Source, PeekableSource};
use porus::io::read::{fread, Consumer};

#[derive(Clone, Copy)]
enum Suit { S, H, C, D }

fn fmt(suit: Suit) -> u8 {
    match suit {
        Suit::S => b'S',
        Suit::H => b'H',
        Suit::C => b'C',
        Suit::D => b'D',
    }
}

impl Default for Suit {
    fn default() -> Self {
        Suit::S
    }
}

#[derive(Default, Clone, Copy)]
struct Card(Suit, u8);

fn cmp(a: &Card, b: &Card) -> bool {
    a.1 < b.1
}

impl<'a> Consumer for &'a mut Suit {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>) {
        let mut suit = 0;
        fread(s, Char(&mut suit));
        *self =
            match suit {
                b'S' => Suit::S,
                b'H' => Suit::H,
                b'C' => Suit::C,
                b'D' => Suit::D,
                _ => panic!(),
            }
    }
}

impl <'a> Consumer for &'a mut Card {
    fn consume<I : Source>(self, s: &mut PeekableSource<I>) {
        fread(s, &mut self.0);
        fread(s, &mut self.1);
    }
}

fn stable(b: bool) -> &'static str {
    if b {
        "Stable"
    } else {
        "Not stable"
    }
}

fn solve() {
    let n : isize = read!();
    let a = &mut Array::<Card>::new_from_iter((0..n).map(|_| read!()));

    let bi = &mut Array::<isize>::new_from_iter(0..n);
    list::bubble_sort(bi, &|i, j| cmp(&a[*i], &a[*j]));
    writelnf!("{}", join(f!(" "), list::iter(bi).map(|i| f!("{:c}{:d}", fmt(a[i].0), a[i].1))));
    writelnf!("{:s}", stable(list::is_stable_sort(a, &cmp, bi)));

    let si = &mut Array::<isize>::new_from_iter(0..n);
    list::selection_sort(si, &|i, j| cmp(&a[*i], &a[*j]));
    writelnf!("{}", join(f!(" "), list::iter(si).map(|i| f!("{:c}{:d}", fmt(a[i].0), a[i].1))));
    writelnf!("{:s}", stable(list::is_stable_sort(a, &cmp, si)));
}
