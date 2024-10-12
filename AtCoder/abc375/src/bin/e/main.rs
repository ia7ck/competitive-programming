use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    let b_sum = ab.iter().map(|&(_, b)| b).sum::<usize>();
    if b_sum % 3 != 0 {
        println!("-1");
        return;
    }

    for &(_, b) in &ab {
        if b > b_sum / 3 {
            println!("-1");
            return;
        }
    }

    const INF: usize = usize::MAX;
    let mut dp = vec![vec![INF; 501]; 501];
    match ab[0] {
        (1, b) => {
            dp[b][0] = 0;
            dp[0][b] = 1;
            dp[0][0] = 1;
        }
        (2, b) => {
            dp[b][0] = 1;
            dp[0][b] = 0;
            dp[0][0] = 1;
        }
        (3, b) => {
            dp[b][0] = 1;
            dp[0][b] = 1;
            dp[0][0] = 0;
        }
        _ => unreachable!(),
    }
    for &(a, b) in &ab[1..] {
        let mut new_dp = vec![vec![INF; 501]; 501];
        for t1 in 0..=500 {
            for t2 in 0..=500 {
                if dp[t1][t2] == INF {
                    continue;
                }
                match a {
                    1 => {
                        if t1 + b <= 500 {
                            new_dp[t1 + b][t2] = new_dp[t1 + b][t2].min(dp[t1][t2]);
                        }
                        if t2 + b <= 500 {
                            new_dp[t1][t2 + b] = new_dp[t1][t2 + b].min(dp[t1][t2] + 1);
                        }
                        new_dp[t1][t2] = new_dp[t1][t2].min(dp[t1][t2] + 1);
                    }
                    2 => {
                        if t1 + b <= 500 {
                            new_dp[t1 + b][t2] = new_dp[t1 + b][t2].min(dp[t1][t2] + 1);
                        }
                        if t2 + b <= 500 {
                            new_dp[t1][t2 + b] = new_dp[t1][t2 + b].min(dp[t1][t2]);
                        }
                        new_dp[t1][t2] = new_dp[t1][t2].min(dp[t1][t2] + 1);
                    }
                    3 => {
                        if t1 + b <= 500 {
                            new_dp[t1 + b][t2] = new_dp[t1 + b][t2].min(dp[t1][t2] + 1);
                        }
                        if t2 + b <= 500 {
                            new_dp[t1][t2 + b] = new_dp[t1][t2 + b].min(dp[t1][t2] + 1);
                        }
                        new_dp[t1][t2] = new_dp[t1][t2].min(dp[t1][t2]);
                    }
                    _ => unreachable!(),
                }
            }
        }
        dp = new_dp;
    }

    let ans = dp[b_sum / 3][b_sum / 3];
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
