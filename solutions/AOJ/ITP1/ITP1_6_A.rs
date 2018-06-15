#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let stdout = unsafe { &mut STDOUT };
    let mut n : isize = default();

    read!(&mut n);

    if n <= 0 {
        return;
    }

    let a = array![0isize; n];

    list::iter_ref_mut(a).foreach(|e| { read!(e); });

    let b = slice!(a, [,,-1]);

    io::fwrite(
        stdout,
        io::join(
            move |s| {
                printf!(s, " ");
            },
            list::iter(b).map(
                |e| {
                    move |s| {
                        printf!(s, "%d", e);
                    }
                }
            )
        ));

    printf!(stdout, "\n");
}
