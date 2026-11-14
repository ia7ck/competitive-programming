use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
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
                x: i32,
            };

            seg.set(i, S::new(x));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };

            let p = seg.prod(l..=r);
            if p == S::default() || p.min == p.max {
                println!("Yes");
            } else {
                println!("No");
            }
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
        S {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct S {
    min: i32, // != -1
    max: i32, // != -1
}

impl S {
    fn new(x: i32) -> Self {
        if x == -1 {
            S::default()
        } else {
            Self { min: x, max: x }
        }
    }
}

impl Default for S {
    fn default() -> Self {
        Self {
            min: i32::MAX,
            max: i32::MIN,
        }
    }
}
