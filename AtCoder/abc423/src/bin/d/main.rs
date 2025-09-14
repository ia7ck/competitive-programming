use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        abc: [(u64, u64, u64); n],
    };

    let mut ans = Vec::new();
    let mut m = 0;
    let mut t = 0;
    let mut leave = BinaryHeap::<(Reverse<u64>, u64)>::new();
    for (a, b, c) in abc {
        while m + c > k {
            let (Reverse(s), d) = leave.pop().unwrap();
            assert!(m >= d);
            t = s;
            m -= d;
        }
        t = t.max(a);
        m += c;
        ans.push(t);
        leave.push((Reverse(t + b), c));
    }

    for ans in ans {
        println!("{}", ans);
    }
}
