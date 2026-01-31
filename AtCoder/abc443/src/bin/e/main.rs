use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            c: Usize1,
            s: [Chars; n],
        };

        solve(n, c, s);
    }
}

fn solve(n: usize, c: usize, s: Vec<Vec<char>>) {
    let mut wall = vec![0; n];
    let mut dp = vec![false; n];
    for j in 0..n {
        if s[n - 1][j] == '#' {
            wall[j] = 1;
        }
    }
    dp[c] = true;
    for i in (1..n).rev() {
        let mut new_dp = vec![false; n];
        for j in 0..n {
            if !dp[j] {
                continue;
            }
            if j >= 1 && (s[i - 1][j - 1] == '.' || wall[j - 1] == 0) {
                new_dp[j - 1] = true;
            }
            if s[i - 1][j] == '.' || wall[j] == 0 {
                new_dp[j] = true;
            }
            if j + 1 < n && (s[i - 1][j + 1] == '.' || wall[j + 1] == 0) {
                new_dp[j + 1] = true;
            }
        }
        dp = new_dp;
        for j in 0..n {
            if s[i - 1][j] == '#' && dp[j] == false {
                wall[j] += 1;
            }
        }
    }
    let mut ans = String::new();
    for j in 0..n {
        if dp[j] {
            ans.push('1');
        } else {
            ans.push('0');
        }
    }
    println!("{}", ans);
}
