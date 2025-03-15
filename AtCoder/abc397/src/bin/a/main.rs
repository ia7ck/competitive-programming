use proconio::{input, marker::Bytes};

fn main() {
    input! {
        x: Bytes,
    };

    let a = (x[0] - b'0') * 10 + (x[1] - b'0');
    let b = x[3] - b'0';

    let ans = if a >= 38 {
        1
    } else if a == 37 && b >= 5 {
        2
    } else {
        3
    };
    println!("{}", ans);
}
