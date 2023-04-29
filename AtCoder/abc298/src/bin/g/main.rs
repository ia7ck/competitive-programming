use std::collections::HashMap;

use proconio::input;

const INF: u64 = std::u64::MAX;

// [pi, qi] × [pj, qj] を t 区画に分割
// min(区画内のイチゴ合計) >= m
// max(区画内のイチゴ合計) の最小値を返す
fn solve(
    pi: usize,
    pj: usize,
    qi: usize,
    qj: usize,
    t: usize,
    m: u64,
    cum_sum: &CumulativeSum2D<u64>,
    memo: &mut HashMap<(usize, usize, usize, usize, usize), u64>,
) -> u64 {
    assert!(pi <= qi);
    assert!(pj <= qj);
    assert!(t >= 1);
    if let Some(&ans) = memo.get(&(pi, pj, qi, qj, t)) {
        return ans;
    }
    let s = cum_sum.sum(pi..(qi + 1), pj..(qj + 1));
    let ans = if (qi - pi + 1) * (qj - pj + 1) < t {
        INF
    } else if s < m * t as u64 {
        INF
    } else if t == 1 {
        assert!(s >= m);
        s
    } else {
        let mut a = INF;
        for t2 in 1..t {
            for i in pi..qi {
                // [pi, i], [i + 1, qi]
                let l = solve(pi, pj, i, qj, t2, m, cum_sum, memo);
                let r = solve(i + 1, pj, qi, qj, t - t2, m, cum_sum, memo);
                a = a.min(l.max(r));
            }
            for j in pj..qj {
                // [pj, j], [j + 1, qj]
                let u = solve(pi, pj, qi, j, t2, m, cum_sum, memo);
                let d = solve(pi, j + 1, qi, qj, t - t2, m, cum_sum, memo);
                a = a.min(u.max(d));
            }
        }
        a
    };
    memo.insert((pi, pj, qi, qj, t), ans);
    // eprintln!("pi = {}, pj = {}, qi = {}, qj = {}, t = {}, m = {}, ans = {:?}", pi, pj, qi, qj, t, m, ans);
    ans
}

fn main() {
    input! {
        h: usize,
        w: usize,
        t: usize,
        s: [[u64; w]; h],
    };

    let cum_sum = CumulativeSum2D::new(&s);
    let mut values = Vec::new();
    for i in 0..h {
        for j in 0..w {
            for di in 1..=h {
                for dj in 1..=w {
                    if i + di <= h && j + dj <= w {
                        values.push(cum_sum.sum(i..(i + di), j..(j + dj)));
                    }
                }
            }
        }
    }
    values.sort();
    values.dedup();

    let mut ans = INF;
    for min in values {
        let mut memo = HashMap::new();
        let v = solve(0, 0, h - 1, w - 1, t + 1, min, &cum_sum, &mut memo);
        assert!(v >= min);
        ans = ans.min(v - min);
    }

    assert_ne!(ans, INF);
    println!("{}", ans);
}

use std::ops::{Add, Range, Sub};

pub struct CumulativeSum2D<T> {
    h: usize,
    w: usize,
    cum_sum: Vec<Vec<T>>,
}

impl<T> CumulativeSum2D<T>
where
    T: Clone + Copy + Default + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    pub fn new(grid: &[Vec<T>]) -> Self {
        let h = grid.len();
        assert!(h >= 1);
        let w = grid[0].len();
        for row in grid {
            assert_eq!(row.len(), w);
        }
        let mut cum_sum = grid.to_vec();
        #[allow(clippy::needless_range_loop)]
        for i in 0..h {
            for j in 1..w {
                cum_sum[i][j] = cum_sum[i][j] + cum_sum[i][j - 1];
            }
        }
        for j in 0..w {
            for i in 1..h {
                cum_sum[i][j] = cum_sum[i - 1][j] + cum_sum[i][j];
            }
        }
        Self { h, w, cum_sum }
    }

    pub fn sum(&self, y_range: Range<usize>, x_range: Range<usize>) -> T {
        let (y_start, y_end) = (y_range.start, y_range.end);
        let (x_start, x_end) = (x_range.start, x_range.end);
        if y_start >= y_end || x_start >= x_end {
            return T::default();
        }
        assert!(y_end <= self.h);
        assert!(x_end <= self.w);
        let sum = self.cum_sum[y_end - 1][x_end - 1];
        if y_start >= 1 && x_start >= 1 {
            assert!(
                sum + self.cum_sum[y_start - 1][x_start - 1]
                    >= self.cum_sum[y_start - 1][x_end - 1] + self.cum_sum[y_end - 1][x_start - 1]
            );
            return sum + self.cum_sum[y_start - 1][x_start - 1]
                - self.cum_sum[y_start - 1][x_end - 1]
                - self.cum_sum[y_end - 1][x_start - 1];
        }
        if y_start >= 1 {
            assert_eq!(x_start, 0);
            assert!(sum >= self.cum_sum[y_start - 1][x_end - 1]);
            return sum - self.cum_sum[y_start - 1][x_end - 1];
        }
        if x_start >= 1 {
            assert_eq!(y_start, 0);
            assert!(sum >= self.cum_sum[y_end - 1][x_start - 1]);
            return sum - self.cum_sum[y_end - 1][x_start - 1];
        }
        sum
    }
}
