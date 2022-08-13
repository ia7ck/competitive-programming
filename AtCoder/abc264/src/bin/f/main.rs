use std::collections::VecDeque;

use proconio::{input, marker::Chars};

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        h: usize,
        w: usize,
        row: [u64; h],
        col: [u64; w],
        a: [Chars; h],
    };

    let a: Vec<Vec<u8>> = a
        .into_iter()
        .map(|r| r.into_iter().map(|ch| ch as u8 - '0' as u8).collect())
        .collect();

    let inf = std::u64::MAX;
    let mut dp = vec![vec![[[inf; 2]; 2]; w]; h];
    dp[0][0][0][0] = 0;
    dp[0][0][0][1] = col[0];
    dp[0][0][1][0] = row[0];
    dp[0][0][1][1] = col[0] + row[0];

    let mut que = VecDeque::new();
    que.push_back((0, 0, 0, 0));
    que.push_back((0, 0, 0, 1));
    que.push_back((0, 0, 1, 0));
    que.push_back((0, 0, 1, 1));
    while let Some((i, j, rr, cc)) = que.pop_front() {
        let b = a[i][j] ^ (rr as u8) ^ (cc as u8);
        if i + 1 < h {
            if b == a[i + 1][j] ^ (cc as u8) {
                if chmin!(dp[i + 1][j][0][cc], dp[i][j][rr][cc]) {
                    que.push_back((i + 1, j, 0, cc));
                }
            } else {
                if chmin!(dp[i + 1][j][1][cc], dp[i][j][rr][cc] + row[i + 1]) {
                    que.push_back((i + 1, j, 1, cc));
                }
            }
        }
        if j + 1 < w {
            if b == a[i][j + 1] ^ (rr as u8) {
                if chmin!(dp[i][j + 1][rr][0], dp[i][j][rr][cc]) {
                    que.push_back((i, j + 1, rr, 0));
                }
            } else {
                if chmin!(dp[i][j + 1][rr][1], dp[i][j][rr][cc] + col[j + 1]) {
                    que.push_back((i, j + 1, rr, 1));
                }
            }
        }
    }

    let mut ans = inf;
    for cc in 0..2 {
        for rr in 0..2 {
            chmin!(ans, dp[h - 1][w - 1][cc][rr]);
        }
    }
    println!("{}", ans);
}
