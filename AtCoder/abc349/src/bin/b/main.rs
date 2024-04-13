use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut counter = HashMap::new();
    for c in s {
        *counter.entry(c).or_insert(0) += 1;
    }

    let mut kind = HashMap::new();
    for (_, v) in counter {
        *kind.entry(v).or_insert(0) += 1;
    }

    for (_, v) in kind {
        if v != 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
