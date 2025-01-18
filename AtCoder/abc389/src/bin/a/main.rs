use proconio::{input, marker::{Bytes}};

fn main() {
    input! {
        s: Bytes,
    };

    let a = u32::from(s[0] - b'0');
    let b = u32::from(s[2] - b'0');
    let ans = a * b;
    println!("{}", ans);
}
