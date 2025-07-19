use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h],
        p: [u64; h + w - 1],
    };

    if f(0, h, w, &a, &p) {
        println!("0");
        return;
    }

    let mut ng = 0;
    let mut ok = 200_000 * 1_000_000_000 + 1;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if f(mid, h, w, &a, &p) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn f(x: u64, h: usize, w: usize, a: &Vec<Vec<u64>>, p: &Vec<u64>) -> bool {
    let mut dp = vec![vec![None; w]; h];
    dp[0][0] = Some(x);
    for k in 0..(h + w - 1) {
        for i in k.saturating_sub(w - 1)..=k.min(h - 1) {
            let j = k - i;
            if let Some(x) = dp[i][j] {
                if let Some(y) = (x + a[i][j]).checked_sub(p[k]) {
                    if j + 1 < w {
                        dp[i][j + 1] = dp[i][j + 1].max(Some(y));
                    }
                    if i + 1 < h {
                        dp[i + 1][j] = dp[i + 1][j].max(Some(y));
                    }
                };
            }
        }
    }

    dp[h - 1][w - 1].is_some_and(|y| y + a[h - 1][w - 1] >= p[h + w - 2])
}
