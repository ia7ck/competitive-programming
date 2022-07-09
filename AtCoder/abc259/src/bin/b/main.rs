use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    };

    let theta = d.to_radians();
    let aa = a * theta.cos() - b * theta.sin();
    let bb = a * theta.sin() + b * theta.cos();
    println!("{} {}", aa, bb);
}
