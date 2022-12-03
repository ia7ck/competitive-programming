use mod_int::ModInt998244353;
use proconio::input;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = $a + $b;
    };
}

fn main() {
    input! {
        n: usize,
        p: usize,
    };

    type Mint = ModInt998244353;
    
    let p2 = Mint::from(p) / 100;
    let p1 = Mint::new(1) - p2;

    let mut dp = vec![Mint::new(0); n];
    for i in 1..n {
        add!(dp[i], (dp[i - 1] + 1) * p1);
        if i >= 2 {
            add!(dp[i], (dp[i - 2] + 1) * p2);
        }
    }

    let ans = dp[n - 1] + 1;
    println!("{}", ans.val());
}
