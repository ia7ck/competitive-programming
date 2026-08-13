use ac_library::{Monoid, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u64,
        mut xc: [(u64, u64); n],
    };

    xc.sort_unstable();

    let mut seg = Segtree::<M>::new(n);
    for i in 0..n {
        seg.set(i, S::new(xc[i].1, u64::MAX));
    }
    let mut l = 0;
    let mut ans = u64::MAX;
    for r in 0..n {
        while l < r && xc[r].0 - xc[l].0 > d {
            l += 1;
        }
        if l < r {
            // xr - xl <= d
            let s = seg.prod(l..=r);
            let cost = s.min.saturating_add(s.second_min);
            ans = ans.min(cost);
        }
    }

    if ans == u64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

struct M {}
#[derive(Debug, Clone, Copy, PartialEq)]
struct S {
    min: u64,
    second_min: u64,
}
impl S {
    fn new(min: u64, second_min: u64) -> Self {
        assert!(min <= second_min);
        Self { min, second_min }
    }
}

impl Monoid for M {
    type S = S;

    fn identity() -> Self::S {
        S::new(u64::MAX, u64::MAX)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let &S {
            min: a_min,
            second_min: a_second_min,
        } = a;
        let &S {
            min: b_min,
            second_min: b_second_min,
        } = b;

        if a_min == u64::MAX {
            return *b;
        }
        if b_min == u64::MAX {
            return *a;
        }

        S::new(
            a_min.min(b_min),
            a_min.max(b_min).min(a_second_min).min(b_second_min),
        )
    }
}
