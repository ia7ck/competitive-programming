use std::collections::HashMap;

use proconio::input;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = ($a + $b) % 998244353;
    };
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut dp = vec![vec![HashMap::new(); n + 1]; n];
    for i in 1..n {
        for j in 0..i {
            let d = a[j] - a[i];
            let e = dp[i][2].entry(d).or_insert(0);
            add!(*e, 1);
        }
        for j in 0..i {
            let d = a[j] - a[i];
            for k in 2..=n {
                if let Some(&v) = dp[j][k].get(&d) {
                    let e = dp[i][k + 1].entry(d).or_insert(0);
                    add!(*e, v);
                }
            }
        }
    }

    print!("{}", n); // k = 1
    for k in 2..=n {
        let mut ans = 0_usize;
        for i in 0..n {
            for v in dp[i][k].values() {
                add!(ans, v);
            }
        }
        print!(" {}", ans);
    }
    println!();
}
