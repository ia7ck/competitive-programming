use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(f64, f64); n],
    }

    let mut ans = 0.0;
    let (mut x, mut y) = (0.0, 0.0);
    for (px, py) in points {
        ans += (x - px).hypot(y - py);
        (x, y) = (px, py);
    }
    ans += x.hypot(y);

    println!("{}", ans);
}
