use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut counter = HashMap::new();
    for a in a {
        *counter.entry(a).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (k, v) in counter {
        if v % 2 == 1 {
            ans += k;
        }
    }
    println!("{}", ans);
}
