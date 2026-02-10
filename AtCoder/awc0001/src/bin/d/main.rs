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
        k: usize,
        ab: [(u64, usize); n],
    };

    let mut dp = vec![vec![0; m + 1]; n];
    for i in 0..n {
        let (a, b) = ab[i];
        chmax!(dp[i][m - b], a);
        for last in 0..i {
            if i - last > k {
                continue;
            }
            for j in b..=m {
                chmax!(dp[i][j - b], dp[last][j] + a);
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..=m {
            chmax!(ans, dp[i][j]);
        }
    }

    println!("{}", ans);
}
