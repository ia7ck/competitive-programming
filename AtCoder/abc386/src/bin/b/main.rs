use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut ans = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] == '0' && i + 1 < s.len() && s[i + 1] == '0' {
            i += 2;
            ans += 1;
        } else {
            i += 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}
