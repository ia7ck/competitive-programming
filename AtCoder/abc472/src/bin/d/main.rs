use std::collections::VecDeque;

use proconio::{input, marker::Chars};

use crate::grid_search::around;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    };

    let mut safe_row = vec![false; h];
    for i in 0..h {
        safe_row[i] = s[i].iter().all(|&ch| ch == '.');
    }
    let mut safe_col = vec![false; w];
    for j in 0..w {
        safe_col[j] = (0..h).all(|i| s[i][j] == '.');
    }
    let mut safe = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            safe[i][j] = safe_row[i] && safe_col[j];
        }
    }

    let mut dist = vec![vec![usize::MAX; w]; h];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if safe[i][j] {
                dist[i][j] = 0;
                que.push_back((i, j));
            }
        }
    }
    while let Some((i, j)) = que.pop_front() {
        for (ni, nj) in
            around(i, j)
                .y_range(0..h)
                .x_range(0..w)
                .directions(&[(-1, 0), (0, -1), (1, 0), (0, 1)])
        {
            if s[ni][nj] == '#' {
                continue;
            }
            if dist[ni][nj] == usize::MAX {
                dist[ni][nj] = dist[i][j] + 1;
                que.push_back((ni, nj));
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if dist[i][j] <= k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod grid_search {
    use std::ops::Range;

    pub struct Around<'a> {
        y: usize,
        x: usize,
        y_range: Range<usize>,
        x_range: Range<usize>,
        directions: &'a [(isize, isize)],
    }

    pub fn around<'a>(y: usize, x: usize) -> Around<'a> {
        Around {
            y,
            x,
            y_range: 0..usize::MAX,
            x_range: 0..usize::MAX,
            directions: &[],
        }
    }

    impl<'a> Around<'a> {
        pub fn y_range(self, r: Range<usize>) -> Self {
            Self { y_range: r, ..self }
        }
        pub fn x_range(self, r: Range<usize>) -> Self {
            Self { x_range: r, ..self }
        }
        pub fn directions(self, dirs: &'a [(isize, isize)]) -> Self {
            Self {
                directions: dirs,
                ..self
            }
        }
    }

    impl<'a> Iterator for Around<'a> {
        type Item = (usize, usize);
        fn next(&mut self) -> Option<Self::Item> {
            while let Some((&(dy, dx), rest)) = self.directions.split_first() {
                self.directions = rest;
                match (self.y.checked_add_signed(dy), self.x.checked_add_signed(dx)) {
                    (Some(ny), Some(nx))
                        if self.y_range.contains(&self.y)
                            && self.x_range.contains(&self.x)
                            && self.y_range.contains(&ny)
                            && self.x_range.contains(&nx) =>
                    {
                        return Some((ny, nx));
                    }
                    _ => {}
                }
            }
            None
        }
    }
}
