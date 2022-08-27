use proconio::input;

macro_rules! chmax {
    ($a: expr, $b: expr) => {
        $a = $a.max($b);
    };
}

fn main() {
    input! {
        n: usize,
        txa: [(usize, usize, i64); n],
    };

    let s = 1_000_00;
    let mut bonus = vec![vec![0; 5]; s + 1];
    for (t, x, a) in txa {
        bonus[t][x] = a;
    }
    let mut dp = vec![vec![-1; 5]; s + 1];
    dp[0][0] = 0;
    for t in 0..s {
        for p in 0..5 {
            if dp[t][p] == -1 {
                continue;
            }
            chmax!(dp[t + 1][p], dp[t][p] + bonus[t + 1][p]);
            if p >= 1 {
                chmax!(dp[t + 1][p - 1], dp[t][p] + bonus[t + 1][p - 1]);
            }
            if p + 1 < 5 {
                chmax!(dp[t + 1][p + 1], dp[t][p] + bonus[t + 1][p + 1]);
            }
        }
    }
    let mut ans = -1;
    for p in 0..5 {
        chmax!(ans, dp[s][p]);
    }
    assert_ne!(ans, -1);
    println!("{}", ans);
}
