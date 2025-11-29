use std::collections::{HashMap, HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        xr: [(i64, i64); n],
    };

    let mut h = HashMap::new();
    for (i, &(x, r)) in xr.iter().enumerate() {
        for y in [x - r, x + r] {
            h.entry(y).or_insert(HashSet::new()).insert(i);
        }
    }

    let mut seen = vec![false; n];
    let mut que = VecDeque::new();
    for v in h.values() {
        if v.len() != 1 {
            continue;
        }
        let i = v.iter().next().copied().unwrap();
        if !seen[i] {
            seen[i] = true;
            que.push_back(i);
        }
    }

    let mut ans = 0;
    while let Some(i) = que.pop_front() {
        ans += 1;
        let (x, r) = xr[i];
        for y in [x - r, x + r] {
            h.entry(y).and_modify(|v| {
                v.remove(&i);
                if v.len() == 1 {
                    let j = v.iter().next().copied().unwrap();
                    if !seen[j] {
                        seen[j] = true;
                        que.push_back(j);
                    }
                }
            });
        }
    }

    for v in h.values() {
        if v.len() >= 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
