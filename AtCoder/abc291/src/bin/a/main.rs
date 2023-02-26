use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    };

    for i in 0..s.len() {
        if b'A' <= s[i] && s[i] <= b'Z' {
            println!("{}", i + 1);
            return;
        }
    }

    unreachable!();
}
