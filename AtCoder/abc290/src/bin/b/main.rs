use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Bytes,
    };

    let mut ans = String::new();
    let mut pass = 0;
    for b in s {
        if b == b'o' && pass + 1 <= k {
            ans.push('o');
            pass += 1;
        } else {
            ans.push('x');
        }
    }

    println!("{}", ans);
}
