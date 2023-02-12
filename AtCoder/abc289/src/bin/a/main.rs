use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    };

    for b in s {
        print!("{}", 1 - (b - b'0'));
    }
    println!();
}
