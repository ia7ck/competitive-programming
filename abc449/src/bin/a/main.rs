use std::f64::consts;

use proconio::input;

fn main() {
    input! {
        d: f64,
    };

    let ans = consts::PI * (d / 2.0).powi(2);

    println!("{}", ans);
}
