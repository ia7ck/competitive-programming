use mod_int::ModInt998244353;
use proconio::{input, marker::Usize1};

type Mint = ModInt998244353;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = $a + $b;
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut dp = vec![Mint::new(0); n];
    dp[0] = Mint::new(1);
    for t in 0..k {
        let mut trans = Vec::new();
        for &(x, y) in &edges {
            trans.push(((n * 2 * k + y - t - 1) % n, dp[(n * k + x - t) % n]));
        }
        for (y, v) in trans {
            add!(dp[y], v);
        }
        // eprintln!("{:?}", dp.iter().map(|x| x.val()).collect::<Vec<_>>());
    }

    let mut ans = Mint::new(0);
    for dp in dp {
        ans += dp;
    }
    println!("{}", ans.val());
}
