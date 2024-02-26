use proconio::input;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = ($a + $b) % 998244353;
    };
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    const L: usize = 500;
    let mut dp = vec![0; n + 1];
    let mut dp_small = vec![vec![0; L]; n + 1];
    dp[1] = 1;
    for (i, &x) in (1..=n).zip(&a) {
        if x >= L {
            for j in ((i + x)..=n).step_by(x) {
                add!(dp[j], dp[i]);
            }
        } else {
            add!(dp_small[i][x], dp[i]);
        }

        for y in 1..L {
            if i + y <= n {
                add!(dp[i + y], dp_small[i][y]);
                add!(dp_small[i + y][y], dp_small[i][y]);
            }
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        add!(ans, dp[i]);
    }
    println!("{}", ans);
}
