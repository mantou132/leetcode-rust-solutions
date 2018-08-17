#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let n: isize = read!();
    let a = &mut Array::<isize>::new_from_iter((0..n).map(|_| read!()));

    let gaps =
        static_array![797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1];

    let mut skip = 0;
    while (gaps[skip] > n) && (gaps[skip] != 1) {
        skip += 1;
    }
    let g = slice!(gaps, [skip,]);

    let count = list::shell_sort(a, &PartialOrd::lt, g);

    writelnf!("{:d}", collection::size(g));
    writelnf!("{}", join(f!(" "), list::iter(g).map(|e| f!("{e:d}"))));

    writelnf!("{count:d}");
    writelnf!("{}", join(f!("\n"), list::iter(a).map(|e| f!("{e:d}"))));
}
