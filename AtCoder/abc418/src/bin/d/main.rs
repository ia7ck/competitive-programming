use proconio::{input};

fn main() {
    input! {
        _n: usize,
        t: String,
    };

    let mut ans = 0_usize;
    let mut dp0 = 0;
    let mut dp1 = 0;
    for c in t.chars() {
        if c == '0' {
            ans += dp0;
            (dp0, dp1) = (dp1 + 1, dp0);
        } else {
            ans += dp1 + 1;
            (dp0, dp1) = (dp0, dp1 + 1);
        }
    }

    println!("{}", ans);
}
