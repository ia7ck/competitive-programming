extern crate proconio;
use proconio::input;

fn main() {
    input! {
        a: i32,
        mut b: i32,
        mut c: i32,
        k: i32,
    }
    let mut cnt = 0;
    while a >= b {
        b = b * 2;
        cnt += 1;
    }
    while b >= c {
        c = c * 2;
        cnt += 1;
    }
    println!("{}", if cnt <= k { "Yes" } else { "No" });
}
