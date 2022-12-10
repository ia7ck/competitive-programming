use proconio::input;

macro_rules! chmax {
    ($a: expr, $b: expr) => {
        if let Some(x) = $a {
            $a = Some(x.max($b));
        } else {
            $a = Some($b);
        }
    };
}

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    };

    let mut dp = vec![vec![None; k + 1]; d];
    dp[0][0] = Some(0);
    for x in a {
        let mut next = vec![vec![Option::<usize>::None; k + 1]; d];
        for m in 0..d {
            for j in 0..=k {
                if let Some(cur) = dp[m][j] {
                    chmax!(next[m][j], cur);
                    if j + 1 <= k {
                        chmax!(next[(m + x) % d][j + 1], cur + x);
                    }
                }
            }
        }
        dp = next;
    }

    if let Some(ans) = dp[0][k] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
