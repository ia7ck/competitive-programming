use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _n: usize,
        s: Bytes,
    };

    let ok = s.iter().any(|&b| b == b'o') && s.iter().all(|&b| b != b'x');
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
