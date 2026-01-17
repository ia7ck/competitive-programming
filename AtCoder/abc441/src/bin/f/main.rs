use proconio::input;

macro_rules! chmax {
    ($a: expr, $b: expr) => {
        $a = $a.max($b);
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        pv: [(usize, u64); n],
    };

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        let (p, v) = pv[i];
        for j in 0..=m {
            chmax!(dp[i + 1][j], dp[i][j]);
            if j >= 1 {
                chmax!(dp[i + 1][j], dp[i + 1][j - 1]);
            }
            if j >= p {
                chmax!(dp[i + 1][j], dp[i][j - p] + v);
            }
        }
    }
    let mut dp2 = vec![vec![0; m + 1]; n + 1];
    for i in (0..n).rev() {
        let (p, v) = pv[i];
        for j in 0..=m {
            chmax!(dp2[i][j], dp2[i + 1][j]);
            if j >= 1 {
                chmax!(dp2[i][j], dp2[i][j - 1]);
            }
            if j >= p {
                chmax!(dp2[i][j], dp2[i + 1][j - p] + v);
            }
        }
    }
    let best = dp[n][m];

    let mut ans = String::new();
    for i in 0..n {
        let (p, v) = pv[i];
        // dp[..=i], dp2[i+1..]
        let mut excl = 0;
        for j in 0..=m {
            chmax!(excl, dp[i][j] + dp2[i + 1][m - j]);
        }
        let mut incl = 0;
        for j in 0..=(m - p) {
            chmax!(incl, dp[i][j] + v + dp2[i + 1][(m - p) - j]);
        }
        if excl < best {
            ans.push('A');
        } else if incl < best {
            ans.push('C');
        } else {
            ans.push('B');
        }
    }
    println!("{}", ans);
}
