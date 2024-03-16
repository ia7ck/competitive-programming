use std::cmp::Reverse;

use proconio::{input, marker::Usize1};

// TODO: speed up
// https://atcoder.jp/contests/abc345/submissions/51355681

fn main() {
    input! {
        n: usize,
        k: usize,
        balls: [(Usize1, u64); n],
    };

    // maintain top 2
    let mut dp = vec![vec![]; k + 1];
    dp[0] = vec![balls[0]];
    dp[1] = vec![(usize::MAX, 0)];
    for &(c, v) in &balls[1..] {
        let mut new_dp = vec![vec![]; k + 1];
        for j in 0..=k {
            assert!(dp[j].len() <= 2);

            // remove the ball
            if j + 1 <= k {
                for &dp in &dp[j] {
                    new_dp[j + 1].push(dp);
                }
                new_dp[j + 1].sort_by_key(|&(_, acc)| Reverse(acc));
                new_dp[j + 1].dedup_by_key(|&mut (last, _)| last);
                new_dp[j + 1].truncate(2);
            }

            // add the ball
            for &(last, acc) in &dp[j] {
                if last != c {
                    new_dp[j].push((c, acc + v));
                }
            }
            new_dp[j].sort_by_key(|&(_, acc)| Reverse(acc));
            new_dp[j].dedup_by_key(|&mut (last, _)| last);
            new_dp[j].truncate(2);
        }
        dp = new_dp;
    }

    if let Some((_, ans)) = dp[k].get(0) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
