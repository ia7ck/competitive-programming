use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    };

    // dp[i][j] := x[..i] == s[..i], 最も右でtを押したのが x[(i-j)..(i-j+m)] かどうか
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[0][0] = true; // ???
    for i in 0..n {
        for j in 0..m {
            if dp[i][j] && s[i] == t[j] {
                dp[i + 1][j + 1] = true;
            }
        }

        // 上から押す
        dp[i + 1][0] = dp[i + 1].iter().any(|&ok| ok);

        if dp[i + 1][m] {
            for j in 0..m {
                // 下に押していたことにする
                dp[i + 1][j] = true;
            }
        }
    }

    if dp[n][m] {
        println!("Yes");
    } else {
        println!("No");
    }
}
