use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();
    println!("{}", s[n / 2]);
}