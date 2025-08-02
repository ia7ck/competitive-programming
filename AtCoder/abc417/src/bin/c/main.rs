use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    // j - i = a[i] + a[j]
    // a[i] + i = j - a[j]

    let mut ans = 0_usize;
    let mut counter = HashMap::new();
    for j in 0..n {
        if j >= a[j] {
            ans += counter.get(&(j - a[j])).unwrap_or(&0);
        }
        *counter.entry(a[j] + j).or_insert(0) += 1;
    }

    println!("{}", ans);
}
