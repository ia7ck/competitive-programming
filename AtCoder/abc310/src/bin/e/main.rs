use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut zero = 0;
    let mut one = 0;
    if s[0] == '0' {
        zero = 1;
    } else {
        one = 1;
    }
    let mut ans: usize = one;
    for i in 1..n {
        let (zero_, one_) = if s[i] == '0' {
            (1, zero + one)
        } else {
            (one, zero + 1)
        };
        zero = zero_;
        one = one_;
        ans += one;
    }
    println!("{}", ans);
}
