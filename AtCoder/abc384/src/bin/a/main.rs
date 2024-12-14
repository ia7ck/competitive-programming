use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        c1: char,
        c2: char,
        s: Chars,
    };

    let mut ans = String::new();
    for c in s {
        if c == c1 {
            ans.push(c1);
        } else {
            ans.push(c2);
        }
    }

    println!("{}", ans);
}
