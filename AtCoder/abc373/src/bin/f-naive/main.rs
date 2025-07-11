use proconio::input;

fn main() {
    input! {
        n: usize,
        ws: usize,
        items: [(usize, usize); n],
    };

    // dp[i][j] := 品物iまでを使って重み合計j以下の最大うれしさ
    let mut dp = vec![vec![0; ws + 1]; n + 1];
    for (i, &(w, v)) in items.iter().enumerate() {
        for c in 0..=(ws / w) {
            if v * c < c * c {
                continue;
            }
            for j in (w * c)..=ws {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j - w * c] + (v * c - c * c));
            }
        }
    }

    println!("{}", dp[n][ws]);
}
