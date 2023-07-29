use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    let mut ans = Vec::new();
    for i in 0..n {
        for j in 0..m {
            if i + 9 > n || j + 9 > m {
                continue;
            }
            let mut ok = true;
            for di in 0..3 {
                for dj in 0..3 {
                    ok &= s[i + di][j + dj] == '#';
                }
            }
            for d in 0..=3 {
                ok &= s[i + d][j + 3] == '.';
                ok &= s[i + 3][j + d] == '.';
            }
            for d in 5..9 {
                ok &= s[i + d][j + 5] == '.';
                ok &= s[i + 5][j + d] == '.';
            }
            for di in 6..9 {
                for dj in 6..9 {
                    ok &= s[i + di][j + dj] == '#';
                }
            }
            if ok {
                ans.push((i, j));
            }
        }
    }

    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
