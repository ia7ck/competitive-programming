use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let i = s[0] as u8 - b'0';
    let j = s[2] as u8 - b'0';

    if j < 8 {
        println!("{}-{}", i, j + 1);
    } else if i < 8 && j == 8 {
        println!("{}-{}", i + 1, 1);
    } else {
        unreachable!();
    }
}
