use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _n: usize,
        s: Bytes,
    };

    let l = s.iter().position(|&b| b == b'|').unwrap();
    let r = s.iter().rposition(|&b| b == b'|').unwrap();
    let m = s.iter().rposition(|&b| b == b'*').unwrap();

    if l < m && m < r {
        println!("in");
    } else {
        println!("out");
    }
}
