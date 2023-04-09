use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    };

    let b1 = s.iter().position(|&b| b == b'B').unwrap();
    let b2 = s.iter().rposition(|&b| b == b'B').unwrap();
    if b1 % 2 == b2 % 2 {
        println!("No");
        return;
    }

    let r1 = s.iter().position(|&b| b == b'R').unwrap();
    let r2 = s.iter().rposition(|&b| b == b'R').unwrap();
    let k = s.iter().rposition(|&b| b == b'K').unwrap();
    if r1 < k && k < r2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
