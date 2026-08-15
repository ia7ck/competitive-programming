use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let t = s
        .into_iter()
        .map(|s| {
            s.into_iter()
                .map(|ch| ch.to_ascii_lowercase())
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    let mut counter = HashMap::new();
    for t in t {
        *counter.entry(t).or_insert(0) += 1;
    }

    let ans = counter.values().max().copied().unwrap();
    println!("{}", ans);
}
