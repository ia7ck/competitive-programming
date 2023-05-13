use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
        n: u64,
    };

    let mut k = 0;
    for i in 0..s.len() {
        if s[i] == b'1' {
            k += 1 << (s.len() - i - 1);
        }
    }
    if k > n {
        println!("-1");
        return;
    }
    for i in 0..s.len() {
        if s[i] == b'?' {
            let a = 1 << (s.len() - i - 1);
            if k + a <= n {
                k += a;
            }
        }
    }
    println!("{}", k);
}
