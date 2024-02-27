use proconio::input;

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn main() {
    input! {
        n: usize,
        d: usize,
        w: [u64; n],
    };

    const INF: u64 = u64::MAX / 2;
    let mut dp = vec![vec![INF; 1 << n]; d + 1];
    for set in 0..(1 << n) {
        let mut sum = 0;
        for i in 0..n {
            if set >> i & 1 == 1 {
                sum += w[i];
            }
        }
        dp[1][set] = sum.pow(2);
    }
    for k in 2..=d {
        // https://cp-algorithms.com/algebra/all-submasks.html
        for set in 0..(1 << n) {
            let mut exclude = set;
            while exclude > 0 {
                chmin!(dp[k][set], dp[1][exclude] + dp[k - 1][set ^ exclude]);
                exclude = (exclude - 1) & set;
            }
        }
    }

    // V = dp[d][(1<<n)-1]/d - (sum(w)/d).pow(2)
    //   = (d*dp[d][(1<<n)-1] - sum(w).pow(2)) / d.pow(2)

    let denom = (d as u64) * dp[d][(1 << n) - 1] - w.iter().sum::<u64>().pow(2);
    let numer = d.pow(2);
    let ans = denom as f64 / numer as f64;
    println!("{}", ans);
}
