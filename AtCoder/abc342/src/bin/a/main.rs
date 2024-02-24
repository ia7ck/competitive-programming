use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };

    let mut count = HashMap::new();
    for &ch in &s {
        *count.entry(ch).or_insert(0) += 1;
    }

    let mut single = None;
    for (ch, v) in count {
        if v == 1 {
            single = Some(ch);
            break;
        }
    }
    let single = single.unwrap();

    let ans = s.iter().position(|&ch| ch == single).unwrap() + 1;
    println!("{}", ans);
}
