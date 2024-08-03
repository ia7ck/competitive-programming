use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    let mut dp = vec![None; 3];
    match s[0] {
        'R' => {
            dp[0] = Some(0);
            dp[1] = Some(1);
        }
        'P' => {
            dp[1] = Some(0);
            dp[2] = Some(1);
        }
        'S' => {
            dp[2] = Some(0);
            dp[0] = Some(1);
        }
        _ => unreachable!(),
    }

    for &c in &s[1..] {
        let mut new_dp = vec![Option::<usize>::None; 3];
        for i in 0..3 {
            if let Some(dp) = dp[i] {
                for j in 0..3 {
                    if i == j {
                        continue;
                    }
                    match c {
                        'R' => {
                            if j == 0 {
                                // draw
                                new_dp[j] = Some(new_dp[j].map_or(dp, |x| x.max(dp)));
                            }
                            if j == 1 {
                                // win
                                new_dp[j] = Some(new_dp[j].map_or(dp + 1, |x| x.max(dp + 1)));
                            }
                        }
                        'P' => {
                            if j == 1 {
                                // draw
                                new_dp[j] = Some(new_dp[j].map_or(dp, |x| x.max(dp)));
                            }
                            if j == 2 {
                                // win
                                new_dp[j] = Some(new_dp[j].map_or(dp + 1, |x| x.max(dp + 1)));
                            }
                        }
                        'S' => {
                            if j == 2 {
                                // draw
                                new_dp[j] = Some(new_dp[j].map_or(dp, |x| x.max(dp)));
                            }
                            if j == 0 {
                                // win
                                new_dp[j] = Some(new_dp[j].map_or(dp + 1, |x| x.max(dp + 1)));
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        dp = new_dp;
    }

    let mut ans = 0;
    for i in 0..3 {
        if let Some(dp) = dp[i] {
            ans = ans.max(dp);
        }
    }
    println!("{}", ans);
}
