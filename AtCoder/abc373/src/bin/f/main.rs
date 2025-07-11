use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        items: [(usize, usize); n],
    };

    let mut v_by_w = vec![vec![]; w + 1];
    for (w, v) in items {
        v_by_w[w].push(v);
    }

    // dp[i][j] := 重みi以下の品物のみを使って重み合計j以下の最大うれしさ
    let mut dp = vec![vec![0; w + 1]; w + 1];
    for i in 1..=w {
        let mut h = 0;
        let mut heap = v_by_w[i].iter().map(|&v| v - 1).collect::<BinaryHeap<_>>();
        for c in 0..=(w / i) {
            for j in (i * c)..=w {
                dp[i][j] = dp[i][j].max(
                    dp[i - 1][j - i * c] + h,
                );
            }

            let Some(v) = heap.pop() else {
                continue;
            };
            h += v;
            heap.push(v.saturating_sub(2));
        }
    }

    println!("{}", dp[w][w]);
}
