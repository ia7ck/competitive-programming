use proconio::{input, marker::Usize1};

use cumulative_sum_2d::CumulativeSum2D;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(Usize1, Usize1); n],
    };

    let mut table = vec![vec![0; w]; h];
    for (a, b) in ab {
        table[a][b] += 1;
    }
    let cum_sum = CumulativeSum2D::new(&table);

    let check = |i: usize, j: usize, size: usize| {
        i + size <= h && j + size <= w && cum_sum.sum(i..(i + size), j..(j + size)) == 0
    };

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let mut ok = 0;
            let mut ng = h.max(w) + 1;
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if check(i, j, mid) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ans += ok;
        }
    }
    println!("{}", ans);
}
