use std::collections::BTreeSet;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        q: usize,
        s: Chars,
        t: Chars,
        queries: [(Usize1, Usize1); q],
    };

    let mut begin = BTreeSet::new();
    for i in 0..s.len() {
        let end = i + t.len();
        if end <= s.len() && s[i..end] == t {
            begin.insert(i);
        }
    }
    for (l, r) in queries {
        if r - l + 1 < t.len() {
            println!("No");
            continue;
        }
        if begin.range(l..=(r - t.len() + 1)).next().is_some() {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
