use proconio::input;

// (q + 0.9x) / (1 + 0.9y) - 1200 / sqrt(k + 1)
fn r(q: f64, x: f64, y: f64, k: usize) -> f64 {
    (q + 0.9 * x) / (1.0 + 0.9 * y) - 1200.0 / ((k + 1) as f64).sqrt()
}

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    };

    let mut dp = vec![None; n + 1];
    dp[0] = Some((0.0_f64, 0.0_f64));
    for p in p {
        for k in (1..=n).rev() {
            match (dp[k - 1], dp[k]) {
                (None, None) => {}
                (None, Some(_)) => unreachable!(),
                (Some((x, y)), None) => {
                    dp[k] = Some((p + 0.9 * x, 1.0 + 0.9 * y));
                }
                (Some((x, y)), Some((xx, yy))) => {
                    if r(p, x, y, k - 1) > xx / yy - 1200.0 / (k as f64).sqrt() {
                        dp[k] = Some((p + 0.9 * x, 1.0 + 0.9 * y));
                    }
                }
            }
        }
    }

    let mut ans = f64::MIN;
    for k in 1..=n {
        let (x, y) = dp[k].unwrap();
        ans = ans.max(x / y - 1200.0 / (k as f64).sqrt());
    }
    println!("{}", ans);
}
