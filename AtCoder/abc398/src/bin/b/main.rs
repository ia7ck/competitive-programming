use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: [u32; 7],
    };

    for c in a.iter().combinations(5) {
        if full_house(&c) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

fn full_house(a: &Vec<&u32>) -> bool {
    assert_eq!(a.len(), 5);
    let mut count = HashMap::new();
    for &x in a {
        *count.entry(x).or_insert(0) += 1;
    }
    let mut count = count.values().cloned().collect::<Vec<_>>();
    count.sort();
    count == vec![2, 3]
}
