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
        a: [i64; n],
    };

    let inf = std::i64::MAX;
    let mut dp = vec![-inf; m + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut next = dp.clone();
        for j in 0..=m {
            if dp[j] == -inf {
                continue;
            }
            chmax!(next[j], dp[j]);
            if j + 1 <= m {
                chmax!(next[j + 1], dp[j] + a[i] * (j + 1) as i64);
            }
        }
        dp = next;
    }
    assert_ne!(dp[m], -inf);
    println!("{}", dp[m]);
}
