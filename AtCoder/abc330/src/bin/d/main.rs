use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut ans = 0_u64;
    {
        let mut up = vec![0; n];
        for i in 0..n {
            let mut left = 0;
            for j in 0..n {
                // ┘
                if s[i][j] == 'o' {
                    ans += left * up[j];
                    left += 1;
                }
            }
            let mut right = 0;
            for j in (0..n).rev() {
                // └
                if s[i][j] == 'o' {
                    ans += right * up[j];
                    right += 1;
                }
            }

            for j in 0..n {
                if s[i][j] == 'o' {
                    up[j] += 1;
                }
            }
        }
    }
    {
        let mut down = vec![0; n];
        for i in (0..n).rev() {
            let mut left = 0;
            for j in 0..n {
                // ┐
                if s[i][j] == 'o' {
                    ans += left * down[j];
                    left += 1;
                }
            }
            let mut right = 0;
            for j in (0..n).rev() {
                // ┌
                if s[i][j] == 'o' {
                    ans += right * down[j];
                    right += 1;
                }
            }

            for j in 0..n {
                if s[i][j] == 'o' {
                    down[j] += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
