use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [u32; n],
        l1: u32,
        c1: u64,
        k1: usize,
        l2: u32,
        c2: u64,
        k2: usize,
    };

    let mut dp = vec![usize::MAX; k1 + 1];
    dp[0] = 0; // ???
    for &d in &d {
        let mut new_dp = vec![usize::MAX; k1 + 1];
        for i in 0..=k1 {
            if dp[i] == usize::MAX {
                continue;
            }
            for j in 0..=k1 {
                if i + j > k1 {
                    continue;
                }
                // l1 <= 100000, j <= 1000
                let r = d.saturating_sub(l1 * j as u32);
                new_dp[i + j] = new_dp[i + j].min(dp[i] + ((r + l2 - 1) / l2) as usize);
            }
        }
        dp = new_dp;
    }

    let mut ans = Option::<u64>::None;
    for i in 0..=k1 {
        if dp[i] > k2 {
            continue;
        }
        let x = c1 * i as u64 + c2 * dp[i] as u64;
        ans = match ans.take() {
            None => Some(x),
            Some(y) => Some(x.min(y)),
        }
    }
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
