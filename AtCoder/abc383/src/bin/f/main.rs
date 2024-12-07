use std::vec;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: usize,
        k: usize,
        items: [(usize, usize, Usize1); n],
    };

    let mut items_by_c = vec![vec![]; n];
    for (p, u, c) in items {
        items_by_c[c].push((p, u));
    }

    let mut dp = vec![Option::<usize>::None; x + 1];
    dp[0] = Some(0);
    for c in 0..n {
        let mut new_dp = dp.clone();
        for &(p, u) in &items_by_c[c] {
            for q in (0..x).rev() {
                if q + p <= x {
                    if let Some(v) = dp[q] {
                        new_dp[p + q] = new_dp[p + q].max(Some(u + v + k));
                    }
                    if let Some(v) = new_dp[q] {
                        new_dp[p + q] = new_dp[p + q].max(Some(u + v));
                    }
                }
            }
        }

        dp = new_dp;
    }

    let mut ans = 0;
    for i in 0..=x {
        if let Some(v) = dp[i] {
            ans = ans.max(v);
        }
    }
    println!("{}", ans);
}
