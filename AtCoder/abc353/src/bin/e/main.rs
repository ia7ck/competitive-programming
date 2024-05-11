use std::collections::HashMap;

use proconio::{input, marker::Chars};
use rolling_hash::RollingHash;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut counter = HashMap::new();
    let mut ans = 0;
    for i in (0..n).rev() {
        let rh = RollingHash::from_iter(s[i].iter().copied());
        for j in 1..=s[i].len() {
            if let Some(&c) = counter.get(&rh.hash(0..j)) {
                ans += j * c;
                ans -= (j - 1) * c;
            }
        }
        for j in 1..=s[i].len() {
            *counter.entry(rh.hash(0..j)).or_insert(0) += 1;
        }
    }
    println!("{}", ans);
}
