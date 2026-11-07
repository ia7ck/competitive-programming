use std::cmp::Reverse;

use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut a2b = vec![vec![]; n];
    for &(a, b) in &ab {
        a2b[a].push(b);
    }
    let mut vote = Segtree::<M>::new(n);
    for i in 0..n {
        let len = a2b[i].len();
        vote.set(i, (len, Reverse(i)));
    }

    for k in 0..n {
        let (c, _) = vote.get(k);
        vote.set(k, (c - a2b[k].len(), Reverse(k)));
        for &b in &a2b[k] {
            let (c, _) = vote.get(b);
            vote.set(b, (c + 1, Reverse(b)));
        }

        let (_, Reverse(ans)) = vote.all_prod();
        println!("{}", ans + 1);

        vote.set(k, (c, Reverse(k)));
        for &b in &a2b[k] {
            let (c, _) = vote.get(b);
            vote.set(b, (c - 1, Reverse(b)));
        }
    }
}

struct M;
impl Monoid for M {
    type S = (usize, Reverse<usize>);

    fn identity() -> Self::S {
        (0, Reverse(0))
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
}
