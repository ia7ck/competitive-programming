use proconio::{input};

fn main() {
    input! {
        n: i64,
    };

    let m: i64= 998244353;
    let mut x = n % m;
    if x < 0 {
        x += m;
    }
    assert!(0 <= x && x < m);
    println!("{}", x);
}