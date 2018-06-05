#![feature(proc_macro_non_items)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let (stdin, stdout) = (&mut io::stdin(), &mut io::stdout());
    let mut n : isize = default();

    scanf!(stdin, "%d", &mut n);

    let vec = array![0isize; 0];

    for _ in 0..n {
        let mut op : isize = default();
        scanf!(stdin, " %d", &mut op);
        match op {
            0 => {
                let mut x : isize = default();
                scanf!(stdin, " %d", &mut x);
                Stack::push(vec, x);
            },
            1 => {
                let mut p : isize = default();
                scanf!(stdin, " %d", &mut p);
                printf!(stdout, "%d\n", vec[p]);
            },
            2 => {
                Stack::pop(vec);
            },
            _ => {
            },
        }
    }
}
