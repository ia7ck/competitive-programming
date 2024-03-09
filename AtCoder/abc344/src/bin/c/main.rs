use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        m: usize,
        b: [u64; m],
        l: usize,
        c: [u64; l],
        q: usize,
        x: [u64; q],
    };

    let mut set = HashSet::new();
    for &a in &a {
        for &b in &b {
            for &c in &c {
                set.insert(a + b + c);
            }
        }
    }
    for x in x {
        if set.contains(&x) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
