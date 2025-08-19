use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
    };

    const INF: usize = usize::MAX / 2;
    // dp[s] := sum(a[..i]) % m = s になる最小操作回数
    let mut dp = vec![INF; m];
    dp[0] = 0;
    for i in 0..l {
        // cost[j] := a[i], a[i+l], ... を mod m で j にする最小操作回数
        let mut cost = vec![0; m];
        for j in 0..m {
            for k in (i..n).step_by(l) {
                if a[k] <= j {
                    cost[j] += j - a[k];
                } else {
                    cost[j] += m - a[k] + j;
                }
            }
        }

        let mut new_dp = vec![INF; m];
        for s in 0..m {
            for j in 0..m {
                let t = (s + j) % m;
                new_dp[t] = new_dp[t].min(dp[s] + cost[j]);
            }
        }
        dp = new_dp;
    }

    println!("{}", dp[0]);
}
