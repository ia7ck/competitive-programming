use proconio::{input, marker::Bytes};

const M: usize = 998_244_353;
macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = ($a + $b) % M;
    };
}

fn main() {
    input! {
        s: Bytes,
    };

    let mut dp = vec![0; 3];
    for c in s {
        let c = usize::from(c - b'a');
        let mut new_dp = dp.clone();
        for last in 0..3 {
            if c != last {
                add!(new_dp[c], dp[last]);
            }
        }
        add!(new_dp[c], 1);
        dp = new_dp;
    }

    let mut ans = 0;
    for dp in dp {
        add!(ans, dp);
    }
    println!("{}", ans);
}
