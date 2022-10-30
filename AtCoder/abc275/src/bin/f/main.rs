use proconio::input;

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a = $b;
        }
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };

    let inf = std::usize::MAX;
    let mut dp = vec![vec![inf; 2]; m + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let mut next = vec![vec![inf; 2]; m + 1];
        for j in 0..=m {
            if dp[j][0] < inf {
                chmin!(next[j][1], dp[j][0] + 1);
                if j + a[i] <= m {
                    chmin!(next[j + a[i]][0], dp[j][0]);
                }
            }
            if dp[j][1] < inf {
                chmin!(next[j][1], dp[j][1]);
                if j + a[i] <= m {
                    chmin!(next[j + a[i]][0], dp[j][1]);
                }
            }
        }
        dp = next;
    }

    for x in 1..=m {
        let ans = dp[x][0].min(dp[x][1]);
        if ans == inf {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}
