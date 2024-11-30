use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        d: usize,
        s: Chars,
    };

    let mut ans = d;
    for c in s {
        if c == '.' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
