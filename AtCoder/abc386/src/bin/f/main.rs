use proconio::{input, marker::Chars};

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    };

    const INF: usize = usize::MAX / 2;
    let mut dp = vec![vec![INF; k * 2 + 1]; s.len() + 1];
    dp[0][k] = 0;
    for i in 0..=s.len() {
        for j in 0..=(k * 2) {
            if dp[i][j] == INF {
                continue;
            }
            if j + 1 <= k * 2 {
                // i == s.len() のときにもここが実行されることが重要
                chmin!(dp[i][j + 1], dp[i][j] + 1);
            }
            if i < s.len() && j >= 1 {
                chmin!(dp[i + 1][j - 1], dp[i][j] + 1);
            }
            // i + (j - k)
            let p = i.wrapping_add(j.wrapping_sub(k));
            if i < s.len() && p < t.len() {
                chmin!(dp[i + 1][j], dp[i][j] + usize::from(s[i] != t[p]));
            }
        }
    }

    // k + (t.len() - s.len())
    let j = k.wrapping_add(t.len().wrapping_sub(s.len()));
    if j <= k * 2 && dp[s.len()][j] <= k {
        println!("Yes");
    } else {
        println!("No");
    }
}
