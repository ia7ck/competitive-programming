use proconio::input;

macro_rules! chmax {
    ($a: expr, $b: expr) => {
        $a = $a.max($b);
    };
}

fn main() {
    input! {
        n: usize,
        v_max: usize,
        c: usize,
        vw: [(usize, usize); n],
    };

    let mut dp = vec![0; v_max + 1];
    for (v, w) in vw {
        for u in (0..=v_max).rev() {
            if u + v <= v_max {
                chmax!(dp[u + v], dp[u] + w + c);
            }
        }
        for u in 0..=v_max {
            if u + v <= v_max {
                chmax!(dp[u + v], dp[u] + w);
            }
        }
    }

    let ans = dp.iter().max().copied().unwrap();
    println!("{}", ans);
}
