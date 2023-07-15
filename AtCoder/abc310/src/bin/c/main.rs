use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut set = HashSet::new();
    for s in s {
        let mut t = s.clone();
        t.reverse();
        if set.contains(&s) || set.contains(&t) {
            continue;
        }
        set.insert(s);
    }
    println!("{}", set.len());
}
