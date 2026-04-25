use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
    };

    let mut counter = HashMap::new();
    for &a in &a {
        *counter.entry(a).or_insert(0_u64) += 1;
    }

    let total = a.iter().sum::<u64>();
    let mut del = counter.iter().map(|(&k, &v)| k * v).collect::<Vec<_>>();
    del.sort_unstable();
    del.reverse();
    let ans = total - del.iter().take(k).sum::<u64>();

    println!("{}", ans);
}
