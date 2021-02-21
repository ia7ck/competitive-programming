extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        pts: [(f64, f64, f64); n],
    }
    let mut ng = 0.0;
    let mut ok = 1e9;
    for _ in 0..100 {
        let t = (ng + ok) / 2.0;
        let mut xs = pts
            .iter()
            .map(|(x, _, c)| (x - 1.0 / c * t, x + 1.0 / c * t))
            .collect::<Vec<_>>();
        xs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let ok1 = xs[1..].iter().all(|&(l, _)| l <= xs[0].1);
        let mut ys = pts
            .iter()
            .map(|(_, y, c)| (y - 1.0 / c * t, y + 1.0 / c * t))
            .collect::<Vec<_>>();
        ys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let ok2 = ys[1..].iter().all(|&(t, _)| t <= ys[0].1);
        if ok1 && ok2 {
            ok = t;
        } else {
            ng = t;
        }
    }
    println!("{}", ok);
}
