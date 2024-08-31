use std::collections::HashMap;

use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        coins: [(Usize1, Usize1); n],
    };

    let mut pos = vec![vec![]; h];
    for &(r, c) in &coins {
        pos[r].push(c);
    }
    for i in 0..h {
        pos[i].sort();
    }

    struct M;
    impl Monoid for M {
        type S = (usize, (usize, usize));

        fn identity() -> Self::S {
            (0, (0, 0))
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }

    let mut dp = Segtree::<M>::new(w);
    let mut prev = HashMap::new();
    for i in 0..h {
        for &j in &pos[i] {
            let (x, p) = dp.prod(0..=j);
            dp.set(j, (x + 1, (i, j)));
            prev.insert((i, j), p);
        }
    }

    let (ans, p) = dp.all_prod();
    println!("{}", ans);
    let mut route = vec![(h - 1, w - 1), p];
    let (mut i, mut j) = p;
    while let Some(&(ni, nj)) = prev.get(&(i, j)) {
        if (i, j) == (ni, nj) {
            break;
        }
        route.push((ni, nj));
        (i, j) = (ni, nj);
    }
    route.push((0, 0));
    route.reverse();
    let mut ans = String::new();
    for w in route.windows(2) {
        let (i, j) = w[0];
        let (i2, j2) = w[1];
        assert!(i <= i2);
        assert!(j <= j2);
        ans.push_str("D".repeat(i2 - i).as_str());
        ans.push_str("R".repeat(j2 - j).as_str());
    }
    println!("{}", ans);
}
