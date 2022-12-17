use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    let mut inner = false;
    let mut ans = String::new();
    for ch in s {
        if ch == '"' {
            inner = !inner;
        }
        if ch == ',' && !inner {
            ans.push('.');
        } else {
            ans.push(ch);
        }
    }
    println!("{}", ans);
}
