use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    };

    let mut dp = vec![vec![None; s + 1]; n + 1];
    dp[0][0] = Some(0);
    for i in 0..n {
        let (a, b) = ab[i];
        for t in 0..s {
            if let Some(_) = dp[i][t] {
                if t + a <= s {
                    dp[i + 1][t + a] = Some(a);
                }
                if t + b <= s {
                    dp[i + 1][t + b] = Some(b);
                }
            }
        }
    }
    if let Some(_) = dp[n][s] {
        println!("Yes");
        let mut ans = Vec::new();
        let mut t = s;
        for i in (1..=n).rev() {
            let (a, b) = ab[i - 1];
            let last = dp[i][t].unwrap();
            assert!(last <= t);
            if last == a {
                ans.push('H');
            } else if last == b {
                ans.push('T');
            } else {
                unreachable!();
            }
            t -= last;
        }
        assert_eq!(t, 0);
        ans.reverse();
        for ch in ans {
            print!("{}", ch);
        }
        println!();
    } else {
        println!("No");
    }
}
