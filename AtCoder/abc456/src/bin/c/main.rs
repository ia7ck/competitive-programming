use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    const M: usize = 998_244_353;
    let mut ans = 0;
    let mut len = 0;
    let mut prev = '?';
    for c in s {
        if prev == c {
            ans += 1;
            ans %= M;
            len = 1;
        } else {
            ans += len + 1;
            ans %= M;
            len += 1;
        }
        prev = c;
    }

    println!("{}", ans);
}
