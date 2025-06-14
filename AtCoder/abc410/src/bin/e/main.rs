use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        m: usize,
        monsters: [(usize, usize); n],
    };

    // dp[i][j] := i番目までたおして残り体力がjであるときの最大残り魔力
    let mut dp = vec![vec![None; h + 1]; n + 1];
    dp[0][h] = Some(m);
    for (i, &(a, b)) in monsters.iter().enumerate() {
        for j in 0..=h {
            if let Some(m) = dp[i][j] {
                if j >= a {
                    dp[i + 1][j - a] = dp[i + 1][j - a].max(dp[i][j]);
                }
                if m >= b {
                    dp[i + 1][j] = dp[i + 1][j].max(Some(m - b));
                }
            }
        }
    }

    let ans = (0..=n).rev().find(|&i| dp[i].iter().any(Option::is_some)).unwrap();
    println!("{}", ans);
}
