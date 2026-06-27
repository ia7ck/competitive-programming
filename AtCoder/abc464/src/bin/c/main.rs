use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        adb: [(usize, usize, usize); n],
    };

    let mut counter = HashMap::new();
    let mut ab_by_d = vec![vec![]; m + 1];
    for (a, d, b) in adb {
        *counter.entry(a).or_insert(0) += 1;
        ab_by_d[d].push((a, b));
    }

    for j in 1..=m {
        for &(a, b) in &ab_by_d[j] {
            let c = counter.get_mut(&a).unwrap();
            *c -= 1;
            if *c == 0 {
                counter.remove(&a);
            }

            *counter.entry(b).or_insert(0) += 1;
        }

        println!("{}", counter.len());
    }
}
