use proconio::input;
use next_permutation::NextPermutation;

fn main() {
    input! {
        n: usize,
        s: f64,
        t: f64,
        segments: [(f64, f64, f64, f64); n],
    };

    let mut ans: f64 = 1e18;
    let mut ord = (0..n).collect::<Vec<_>>();
    loop {
        let segments = ord.iter().map(|&i| segments[i]).collect::<Vec<_>>();
        for bits in 0..(1 << n) {
            let mut cost = 0.0;
            let (mut x, mut y) = (0.0, 0.0);
            for i in 0..n {
                let (px, py, qx, qy) = segments[i];
                if bits >> i & 1 == 1 {
                    // (x, y) -> (px, py) -> (qx, qy)
                    cost += (x - px).hypot(y - py) / s + (qx - px).hypot(qy - py) / t;
                    (x, y) = (qx, qy);
                } else {
                    cost += (x - qx).hypot(y - qy) / s + (px - qx).hypot(py - qy) / t;
                    (x, y) = (px, py);
                }
            }
            ans = ans.min(cost);
        }

        if !ord.next_permutation() {
            break;
        }
    }

    println!("{}", ans);
}
