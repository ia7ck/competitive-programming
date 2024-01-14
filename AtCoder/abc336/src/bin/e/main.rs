use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: Bytes,
    };

    let mut ans = 0_u64;
    for s in 1..=9 * 14 {
        let mut dp = vec![vec![vec![0; 2]; s]; s + 1];
        dp[0][0][usize::from(false)] = 1;
        for &d in &n {
            let d = usize::from(d - b'0');
            let mut new_dp = vec![vec![vec![0; 2]; s]; s + 1];
            for digit_sum in 0..=s {
                for rem in 0..s {
                    // mod s
                    for less in [false, true] {
                        let lim = if less { 9 } else { d };
                        for x in 0..=lim {
                            let new_digit_sum = digit_sum + x;
                            if new_digit_sum > s {
                                continue;
                            }
                            let new_rem = (rem * 10 + x) % s;
                            let new_less = less || x < d;
                            new_dp[new_digit_sum][new_rem][usize::from(new_less)] += dp[digit_sum][rem][usize::from(less)];
                        }
                    }
                }
            }
            dp = new_dp;
        }
        ans += dp[s][0][usize::from(false)] + dp[s][0][usize::from(true)];
    }
    println!("{}", ans);
}
