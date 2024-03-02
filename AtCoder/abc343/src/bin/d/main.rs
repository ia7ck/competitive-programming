use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(Usize1, u64); t],
    };

    let mut score = vec![0; n];
    let mut count = HashMap::<u64, usize>::new();
    count.insert(0, n);
    for (a, b) in ab {
        let before = score[a];
        let after = before + b;

        let c = count[&before];
        assert!(c >= 1);
        if c == 1 {
            count.remove(&before);
        } else {
            count.insert(before, c - 1);
        }

        count.entry(after).and_modify(|e| *e += 1).or_insert(1);

        println!("{}", count.len());

        score[a] = after;
    }
}
