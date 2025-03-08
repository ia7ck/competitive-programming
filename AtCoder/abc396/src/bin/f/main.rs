use std::cmp::Reverse;

use proconio::input;
use fenwick_tree::FenwickTree;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };

    let mut inv = {
        let mut ord = (0..n).collect::<Vec<_>>();
        ord.sort_by_key(|&i| Reverse((a[i], i)));
        let mut f = FenwickTree::new(n, 0);
        let mut inv = 0;
        for i in ord {
            inv += f.sum(0..i);
            f.add(i, 1);
        }
        inv
    };

    let mut pos = vec![vec![]; m];
    for i in 0..n {
        pos[a[i]].push(i);
    }

    println!("{}", inv);
    for k in 0..(m - 1) {
        // m - k - 1 -> 0
        let mut sub = 0;
        for (i, &p) in pos[m - k - 1].iter().rev().enumerate() {
            sub += (n - p - 1) - i;
        }
        let mut add = 0;
        for (i, &p) in pos[m - k - 1].iter().enumerate() {
            add += p - i;
        }
        assert!(inv + add >= sub);
        inv = inv + add - sub;
        println!("{}", inv);
    }
}
