use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut counter = HashMap::new();
    for &ch in &s {
        *counter.entry(ch).or_insert(0) += 1;
    }
    let max = counter.values().max().copied().unwrap();

    let mut ans = String::new();
    for ch in s {
        if counter[&ch] == max {
            //
        } else {
            ans.push(ch);
        }
    }

    println!("{}", ans);
}
