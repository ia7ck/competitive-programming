use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };

    let n = s.len();
    s[n - 1] = '4';
    println!("{}", s.into_iter().collect::<String>());
}
