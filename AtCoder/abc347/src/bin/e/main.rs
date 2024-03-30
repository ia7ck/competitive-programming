use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [Usize1; q],
    };

    let mut a = vec![0_usize; n];
    let mut set = HashSet::new();
    let mut last = vec![0_usize; n];
    let mut acc = vec![0; q + 1];
    for i in 0..q {
        let removed = if set.contains(&x[i]) {
            set.remove(&x[i]);
            true
        } else {
            set.insert(x[i]);
            false
        };
        if removed {
            let last = last[x[i]].saturating_sub(1);
            a[x[i]] += acc[i] - acc[last];
        }
        last[x[i]] = i + 1;
        acc[i + 1] = acc[i] + set.len();
    }

    // remove
    for x in set {
        let last = last[x].saturating_sub(1);
        a[x] += acc[q] - acc[last];
    }

    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
