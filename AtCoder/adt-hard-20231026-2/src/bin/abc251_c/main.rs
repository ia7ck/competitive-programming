use std::{collections::HashSet, cmp::Reverse};

use proconio::input;

fn main() {
    input! {
        n: usize,
        submissions: [(String, u32); n],
    };

    let mut original = Vec::new();
    let mut seen = HashSet::new();
    for (i, (s, t)) in submissions.iter().enumerate() {
        if seen.contains(s) {
            continue;
        }
        seen.insert(s.clone());
        original.push((i, s, t));
    }

    // stable
    original.sort_by_key(|(_, _, t)| Reverse(*t));
    let (ans, _, __) = original[0];
    println!("{}", ans + 1);
}
