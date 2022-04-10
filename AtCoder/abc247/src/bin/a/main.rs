use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    println!("0{}{}{}", s[0], s[1], s[2]);
}
