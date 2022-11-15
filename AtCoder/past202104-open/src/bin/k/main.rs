use proconio::input;

macro_rules! chmax {
    ($a: expr, $b: expr) => {
        if $a < $b {
            $a = $b;
        }
    };
}

fn main() {
    input! {
        n: usize,
        pu: [(i32, i32); n],
    };

    let inf = std::i32::MAX;
    let mut dp = vec![-inf; 100];
    dp[0] = 0;
    for (p, u) in pu {
        let mut next = dp.clone();
        for x in 0..100 {
            if dp[x] == -inf {
                continue;
            }
            let bonus = (p + x as i32) / 100 * 20;
            chmax!(next[(p as usize + x) % 100], dp[x] + (u - p + bonus));
        }
        dp = next;
    }
    let ans = dp.iter().max().copied().unwrap();
    assert_ne!(ans, -inf);
    println!("{}", ans);
}
