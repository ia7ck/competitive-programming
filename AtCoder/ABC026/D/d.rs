extern crate proconio;
use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    }

    let f = |t: f64| a * t + b * (c * t * std::f64::consts::PI).sin();

    let mut l = 0.0;
    let mut r = 1000.0;
    for _ in 0..100 {
        let t = (l + r) / 2.0;
        if f(t) < 100.0 {
            l = t;
        } else {
            r = t;
        }
    }
    println!("{}", r);
}
