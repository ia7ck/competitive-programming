use proconio::{input, marker::Chars};

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut dp = vec![0; n + 1];
    for i in 0..n {
        let mut new_dp = vec![usize::MAX; n + 1];
        let mut black = vec![0; n + 1];
        for j in 0..n {
            black[j + 1] = black[j] + usize::from(s[i][j] == '#');
        }
        // for j in 0..=n {
        //     for k in 0..=j {
        //         let left = black[k];
        //         let right = (n - k) - (black[n] - black[k]);
        //         chmin!(new_dp[k], dp[j] + left + right);
        //     }
        // }
        let mut min_dp = dp[n];
        for k in (0..=n).rev() {
            chmin!(min_dp, dp[k]);
            let left = black[k];
            let right = (n - k) - (black[n] - black[k]);
            new_dp[k] = min_dp + left + right;
        }
        dp = new_dp;
    }

    let ans = dp.iter().min().unwrap();
    println!("{}", ans);
}
