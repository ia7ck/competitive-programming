use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let ans = if s.len() % 2 == 1 {
        false
    } else {
        let mut eq = true;
        for i in 0..(s.len() / 2) {
            if s[i * 2] != s[i * 2 + 1] {
                eq = false;
                break;
            }
        }
        let mut count = HashMap::new();
        for &c in &s {
            *count.entry(c).or_insert(0) += 1;
        }
        let mut twice = true;
        for (_, &v) in &count {
            if v != 2 {
                twice = false;
                break;
            }
        }
        eq && twice
    };

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
