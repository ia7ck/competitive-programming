use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    };

    let mut y = HashSet::new();
    for &a in &a {
        y.insert(x + a);
    }

    for &a in &a {
        if y.contains(&a) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
