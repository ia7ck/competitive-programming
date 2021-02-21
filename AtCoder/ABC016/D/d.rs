extern crate proconio;
use proconio::input;

fn main() {
    // 2 -> 2
    // 4 -> 3
    // 6 -> 4
    input! {
        ax: i64,
        ay: i64,
        bx: i64,
        by: i64,
        n: usize,
        pts: [(i64, i64); n],
    }

    fn cross(v: (i64, i64), u: (i64, i64)) -> i64 {
        v.0 * u.1 - v.1 * u.0
    }

    let v = (bx - ax, by - ay);
    let mut cnt = 0;
    for i in 0..n {
        let (x1, y1) = pts[i];
        let (x2, y2) = pts[(i + 1) % n];
        let u = (x2 - x1, y2 - y1);
        if cross(v, (x1 - ax, y1 - ay)) * cross(v, (x2 - ax, y2 - ay)) < 0
            && cross(u, (ax - x1, ay - y1)) * cross(u, (bx - x1, by - y1)) < 0
        {
            cnt += 1;
        }
    }
    println!("{}", cnt / 2 + 1);
}
