use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        s: Bytes,
    };

    for i in 1..n {
        let ans = s.iter().zip(s[i..].iter()).take_while(|(b1, b2)| b1 != b2).count();
        println!("{}", ans);
    }
}
