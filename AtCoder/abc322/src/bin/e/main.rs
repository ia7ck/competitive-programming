use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        plans: [(u64, [usize; k]); n],
    };

    let mut dp = HashMap::new();
    dp.insert(vec![0; k], 0);
    for (c, a) in plans {
        let mut new_dp = dp.clone();
        for (mut state, cost) in dp {
            for i in 0..k {
                state[i] += a[i];
                state[i] = state[i].min(p);
            }
            new_dp
                .entry(state)
                .and_modify(|best| *best = (*best).min(cost + c))
                .or_insert(cost + c);
        }
        dp = new_dp;
    }

    let last = vec![p; k];
    if let Some(ans) = dp.get(&last) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
