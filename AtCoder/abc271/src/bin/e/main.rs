use proconio::input;
use proconio::marker::Usize1;

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        edges: [(Usize1, Usize1, u64); m],
        seq: [Usize1; k],
    };

    let inf = std::u64::MAX / 2;

    // let mut dp = vec![vec![inf; n]; k + 1];
    // dp[0][0] = 0;
    // for i in 0..k {
    //     let (a, b, c) = edges[seq[i]];
    //     for v in 0..n {
    //         dp[i + 1][v] = dp[i][v];
    //         if v == a {
    //             chmin!(dp[i + 1][b], dp[i][a] + c);
    //         }
    //     }
    // }

    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..k {
        let (a, b, c) = edges[seq[i]];
        chmin!(dp[b], dp[a] + c);
    }

    // let ans = dp[k][n - 1];
    let ans = dp[n - 1];
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
