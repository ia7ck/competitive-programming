use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
        n: usize,
        a: [[Chars]; n],
    };

    let fit =
        |i: usize, s: &[char]| -> bool { i + s.len() <= t.len() && t[i..(i + s.len())] == s[..] };

    let mut dp = vec![Option::<usize>::None; t.len() + 1];
    dp[0] = Some(0);
    for a in a {
        let mut new_dp = dp.clone();
        for i in 0..t.len() {
            if let Some(dp) = dp[i] {
                for s in &a {
                    if fit(i, s) {
                        new_dp[i + s.len()] = match new_dp[i + s.len()].take() {
                            Some(x) => Some(x.min(dp + 1)),
                            None => Some(dp + 1),
                        };
                    }
                }
            }
        }
        dp = new_dp;
    }

    if let Some(ans) = dp[t.len()] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
