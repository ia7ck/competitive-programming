use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        q: usize,
    };

    let mut plus = HashMap::<usize, u64>::new();
    for i in 0..n {
        plus.insert(i, a[i]);
    }

    let mut last_op1: Option<u64> = None;
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                x: u64,
            };
            last_op1 = Some(x);
            plus.clear();
        } else if op == 2 {
            input! {
                i: Usize1,
                x: u64,
            };
            *plus.entry(i).or_insert(0) += x;
        } else {
            input! {
                i: Usize1,
            };

            let ans = plus.get(&i).unwrap_or(&0) + last_op1.unwrap_or(0);
            println!("{}", ans);
        }
    }
}
