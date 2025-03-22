use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut count = HashMap::new();
    for &x in &a {
        *count.entry(x).or_insert(0) += 1;
    }

    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_unstable_by_key(|&i| a[i]);
    ord.reverse();
    for i in ord {
        if count[&a[i]] == 1 {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
