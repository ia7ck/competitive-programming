use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let pattern1 = ['H', 'D', 'C', 'S'];
    let pattern2 = [
        'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
    ];

    let mut set = HashSet::new();
    for s in s {
        if pattern1.contains(&s[0]) && pattern2.contains(&s[1]) {
            //
        } else {
            println!("No");
            return;
        }
        set.insert(s);
    }
    if set.len() == n {
        println!("Yes");
    } else {
        println!("No");
    }
}
