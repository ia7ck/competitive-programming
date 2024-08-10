use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut counter = HashMap::new();
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: u32,
            };
            *counter.entry(x).or_insert(0) += 1;
        } else if op == 2 {
            input! {
                x: u32,
            }
            let prev = counter[&x];
            assert!(prev >= 1);
            counter.insert(x, prev - 1);
            if prev - 1 == 0 {
                counter.remove(&x);
            }
        } else {
            println!("{}", counter.len());
        }
    }
}
