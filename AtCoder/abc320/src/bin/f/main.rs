use proconio::input;

macro_rules! update {
    ($a:expr, $b:expr) => {
        match $a {
            None => {
                $a = Some($b);
            }
            Some(v) if v > $b => {
                $a = Some($b);
            }
            _ => {}
        }
    };
}

fn main() {
    input! {
        n: usize,
        h: usize,
        mut xs: [usize; n],
        mut pf: [(u64, usize); n - 1],
    };

    xs.insert(0, 0);
    pf.insert(0, (0, 0));
    let xs = xs;
    let pf = pf;

    let mut dp = vec![vec![None; h + 1]; h + 1];
    for k in 0..=h {
        dp[h][k] = Some(0);
    }
    for i in 0..n {
        let dx = xs[i + 1] - xs[i];
        let (p, f) = pf[i];
        let mut next = vec![vec![None; h + 1]; h + 1];
        for j in 0..=h {
            for k in 0..=h {
                let Some(v) = dp[j][k] else {
                    continue;
                };
                if j >= dx && k + dx <= h {
                    update!(next[j - dx][k + dx], v);
                }
                if (j + f).min(h) >= dx && k + dx <= h {
                    update!(next[(j + f).min(h) - dx][k + dx], v + p);
                }
                if j >= dx && k.saturating_sub(f) + dx <= h {
                    update!(next[j - dx][k.saturating_sub(f) + dx], v + p);
                }
            }
        }
        dp = next;
    }

    let mut ans = u64::MAX;
    for j in 0..=h {
        for k in 0..=j {
            if let Some(v) = dp[j][k] {
                ans = ans.min(v);
            }
        }
    }
    if ans == u64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
