use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u8; n],
    };

    let mut seg = Segtree::<M>::new(n);
    for i in 0..n {
        seg.set(i, S::new(a[i]));
    }

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                i: Usize1,
                x: u8,
            };

            seg.set(i, S::new(x));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };

            let ans = seg.prod(l..=r);
            println!("{}", ans.inv);
        }
    }
}

struct M;
impl Monoid for M {
    type S = S;

    fn identity() -> Self::S {
        S::default()
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let c10 = a.c1 * b.c0;
        let c20 = a.c2 * b.c0;
        let c21 = a.c2 * b.c1;

        S {
            c0: a.c0 + b.c0,
            c1: a.c1 + b.c1,
            c2: a.c2 + b.c2,
            inv: a.inv + b.inv + c10 + c20 + c21,
        }
    }
}

#[derive(Clone, Default)]
struct S {
    c0: usize,
    c1: usize,
    c2: usize,
    inv: usize,
}

impl S {
    fn new(x: u8) -> Self {
        let def = S::default();
        match x {
            0 => S { c0: 1, ..def },
            1 => S { c1: 1, ..def },
            2 => S { c2: 1, ..def },
            _ => unreachable!(),
        }
    }
}
