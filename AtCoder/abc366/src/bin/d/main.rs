use proconio::{input, marker::Usize1};
use cumulative_sum_2d::CumulativeSum2D;

fn main() {
    input! {
        n: usize,
        a: [[[i64; n]; n]; n],
        q: usize,
        queries: [(Usize1, Usize1, Usize1, Usize1, Usize1, Usize1); q],
    };

    let mut b = Vec::new();
    for i in 0..n {
        b.push(CumulativeSum2D::new(&a[i]));
    }

    for (lx, rx, ly, ry, lz, rz) in queries {
        let mut ans = 0;
        for x in lx..=rx {
            ans += b[x].sum(ly..(ry + 1), lz..(rz + 1));
        }
        println!("{}", ans);
    }
}
