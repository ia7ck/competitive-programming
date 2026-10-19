use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
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
                x: u32,
            };

            seg.set(i, S::new(x));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };

            let ans = seg.prod(l..=r);

            println!("{}", ans.run);
        }
    }
}

struct M;
impl Monoid for M {
    type S = S;

    fn identity() -> Self::S {
        S {
            run: 0,
            left: u32::MAX,
            right: u32::MAX,
            prefix: 0,
            suffix: 0,
            size: 0,
        }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.is_empty() {
            return b.clone();
        }

        if b.is_empty() {
            return a.clone();
        }

        let run = if a.right == b.left {
            a.run.max(b.run).max(a.suffix + b.prefix)
        } else {
            a.run.max(b.run)
        };
        let prefix = if a.is_full() && a.right == b.left {
            a.size + b.prefix
        } else {
            a.prefix
        };
        let suffix = if b.is_full() && a.right == b.left {
            a.suffix + b.size
        } else {
            b.suffix
        };

        S {
            run,
            left: a.left,
            right: b.right,
            prefix,
            suffix,
            size: a.size + b.size,
        }
    }
}

#[derive(Clone)]
struct S {
    run: usize,
    left: u32,
    right: u32,
    prefix: usize,
    suffix: usize,
    size: usize,
}

impl S {
    fn new(x: u32) -> Self {
        Self {
            run: 1,
            left: x,
            right: x,
            prefix: 1,
            suffix: 1,
            size: 1,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.prefix == self.size
    }
}
