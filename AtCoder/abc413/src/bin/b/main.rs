use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut set = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                let mut t = s[i].clone();
                t.push_str(s[j].as_str());
                set.insert(t);
            }
        }
    }

    println!("{}", set.len());
}