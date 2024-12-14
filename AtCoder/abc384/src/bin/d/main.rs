use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: u64,
        a: [u64; n],
    };

    let mut t = 0;
    let mut prefix = HashSet::new();
    for &x in &a {
        prefix.insert(t);
        t += x;
    }

    let mut l = 0;
    for &x in &a {
        if prefix.contains(&((s + l) % t)) {
            println!("Yes");
            return;
        }
        l += x;
    }

    println!("No");
}
