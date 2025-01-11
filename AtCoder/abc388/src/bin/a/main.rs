use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let ans = format!("{}UPC", s[0]);
    println!("{}", ans);
}
