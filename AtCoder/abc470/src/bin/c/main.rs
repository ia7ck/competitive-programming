use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut a = vec![0; n];
    let mut pos = HashSet::<usize>::new();
    let mut ans = 0;
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                x: Usize1,
            };

            ans ^= a[x];
            a[x] += 1;
            ans ^= a[x];
            pos.insert(x);
        } else {
            pos.retain(|&p| {
                ans ^= a[p];
                a[p] -= 1;
                ans ^= a[p];
                a[p] > 0
            });
        }

        println!("{}", ans);
    }
}
