use proconio::{
    input,
    marker::{Chars, Usize1},
};

use crate::cumulative_sum_2d::CumulativeSum2D;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Chars; n],
        queries: [(Usize1, Usize1, Usize1, Usize1); q],
    };

    let mut b = vec![vec![0; n - 1]; n - 1];
    for i in 0..(n - 1) {
        for j in 0..(n - 1) {
            for (y, x) in [(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)] {
                if a[y][x] == '#' {
                    b[i][j] = 1;
                }
            }
        }
    }
    let cum = CumulativeSum2D::new(&b);
    for (u, d, l, r) in queries {
        let mut ans = (d - u) * (r - l);
        ans -= cum.sum(u..d, l..r);
        println!("{}", ans);
    }
}

// Bundled
#[allow(unused)]
mod cumulative_sum_2d {
    use std::ops::{Add, Range, Sub};

    pub struct CumulativeSum2D<T> {
        h: usize,
        w: usize,
        cum_sum: Vec<Vec<T>>,
    }

    impl<T> CumulativeSum2D<T>
    where
        T: Clone + Copy + Default + Add<Output = T> + Sub<Output = T>,
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
                return sum + self.cum_sum[y_start - 1][x_start - 1]
                    - self.cum_sum[y_start - 1][x_end - 1]
                    - self.cum_sum[y_end - 1][x_start - 1];
            }
            if y_start >= 1 {
                assert_eq!(x_start, 0);
                return sum - self.cum_sum[y_start - 1][x_end - 1];
            }
            if x_start >= 1 {
                assert_eq!(y_start, 0);
                return sum - self.cum_sum[y_end - 1][x_start - 1];
            }
            sum
        }
    }
}
