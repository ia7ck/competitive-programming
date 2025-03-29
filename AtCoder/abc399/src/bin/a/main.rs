use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };

    let mut ans = 0;
    for i in 0..n {
        if s[i] != t[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
