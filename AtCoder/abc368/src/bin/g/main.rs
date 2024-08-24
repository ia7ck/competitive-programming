use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};
use fenwick_tree::FenwickTree;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
        q: usize,
    };

    let mut f = FenwickTree::new(n, 0);
    for i in 0..n {
        f.add(i, a[i]);
    }
    let mut pos = BTreeSet::new();
    for i in 0..n {
        if b[i] >= 2 {
            pos.insert(i);
        }
    }
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                i: Usize1,
                x: i64,
            };
            f.add(i, x - a[i]);
            a[i] = x;
        } else if op == 2 {
            input! {
                i: Usize1,
                x: i64,
            };
            if b[i] >= 2 {
                pos.remove(&i);
            }
            b[i] = x;
            if b[i] >= 2 {
                pos.insert(i);
            }
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };
            let mut ans = 0;
            ans += a[l];
            let mut l = l + 1;
            while let Some(&new_l) = pos.range(l..).next() {
                assert!(b[new_l] >= 2);
                if new_l > r {
                    break;
                }
                ans += f.sum(l..new_l);
                ans = (ans + a[new_l]).max(ans * b[new_l]);
                l = new_l + 1;
            }
            ans += f.sum(l..(r + 1)); // l..=r
            println!("{}", ans);
        }
    }
}
