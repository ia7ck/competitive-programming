extern crate proconio;
use proconio::input;

fn main() {
    input! {
        mut x: i64,
        k: i64,
        d: i64,
    }
    if x < 0 {
        x = x * (-1);
    }
    if x / d >= k {
        println!("{}", x - k * d);
        return;
    }
    let y = x % d;
    println!("{}", if (k - x / d) % 2 == 0 { y } else { d - y });
}
