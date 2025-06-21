use std::collections::{BTreeMap, HashMap};

use mod_int::ModInt998244353;
use proconio::input;
use segment_tree::SegmentTree;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [[u32; 6]; n],
    };

    let mut dice = BTreeMap::<u32, Vec<usize>>::new();
    for i in 0..n {
        for &x in &a[i] {
            dice.entry(x).or_default().push(i);
        }
    }

    let mut q = HashMap::<u32, Mint>::new();
    let mut seg = SegmentTree::new(n, Mint::new(1), |&a, &b| a * b);
    for i in 0..n {
        seg.set(i, Mint::new(0));
    }
    for (&x, dice) in dice.iter() {
        let mut count = HashMap::<usize, usize>::new();
        for &i in dice {
            *count.entry(i).or_insert(0) += 1;
        }
        for (i, value) in count {
            seg.update(i, |&v| v + value);
        }
        q.insert(x, seg.fold(..) / Mint::new(6).pow(n as u32));
    }

    let mut ans = Mint::new(0);
    let mut y = None;
    for (x, _) in dice {
        let p = match y {
            None => q[&x],
            Some(y) => q[&x] - q[&y],
        };
        ans += Mint::from(x) * p;
        y = Some(x);
    }

    println!("{}", ans.val());
}
