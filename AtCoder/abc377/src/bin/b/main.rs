use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 8],
    };

    let n = 8;
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                continue;
            }
            let yoko = (0..n).all(|k| s[i][k] == '.');
            let tate = (0..n).all(|k| s[k][j] == '.');
            if yoko && tate {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
