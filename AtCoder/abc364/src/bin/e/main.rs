use proconio::input;

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut ab: [(usize, usize); n],
    };

        const INF: usize = usize::MAX;

    // dp[i][j][k] := i個目まで考えてj個食べてk=sum(a)のときのmin(sum(b))
    let mut dp = vec![vec![vec![INF; x + 1]; n + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=n {
            for k in 0..=x {
                if dp[i][j][k] < INF {
                    chmin!(dp[i + 1][j][k], dp[i][j][k]);
                    if j + 1 <= n && k + a <= x {
                        chmin!(dp[i + 1][j + 1][k + a], dp[i][j][k] + b);
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for j in 0..=n {
        for k in 0..=x {
            if dp[n][j][k] <= y {
                ans = ans.max((j + 1).min(n));
            }
        }
    }
    println!("{}", ans);
}
