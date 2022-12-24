use proconio::{input, marker::Bytes};

fn main() {
    input! {
        mut s: Bytes,
    };

    s.reverse();
    let mut i = 0;
    let mut ans = 0;
    while i < s.len() {
        if s[i] == b'0' && i + 1 < s.len() && s[i + 1] == b'0' {
            i += 2;
            ans += 1;
        } else {
            i += 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}
