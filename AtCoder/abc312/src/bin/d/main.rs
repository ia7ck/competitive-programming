use proconio::{input, marker::Chars};

const M: u64 = 998244353;

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a = ($a + $b) % M;
    };
}

fn main() {
    input! {
        s: Chars,
    };

    let mut dp = vec![0_u64; s.len() + 1];
    dp[0] = 1;
    for &ch in &s {
        let mut next = vec![0; s.len() + 1];
        if ch == '(' {
            for i in 0..s.len() {
                next[i + 1] = dp[i];
            }
        } else if ch == ')' {
            for i in 0..s.len() {
                next[i] = dp[i + 1];
            }
        } else {
            // ? -> (
            for i in 0..s.len() {
                next[i + 1] = dp[i];
            }
            // ? -> )
            for i in 0..s.len() {
                add!(next[i], dp[i + 1]);
            }
        }
        dp = next;
    }
    println!("{}", dp[0]);
}
