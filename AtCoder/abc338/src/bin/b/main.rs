use std::{collections::HashMap, cmp::Reverse};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut count = HashMap::new();
    for ch in s {
        *count.entry(ch).or_insert(0) += 1;
    }
    let mut count = count.into_iter().collect::<Vec<_>>();
    count.sort_by_key(|&(ch, x)| (Reverse(x), ch));
    let (ans, _) = count[0];
    println!("{}", ans);
}
