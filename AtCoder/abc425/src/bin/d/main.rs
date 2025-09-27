use proconio::{input, marker::Chars};

use crate::grid_search::around;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    };

    let mut adj = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&[
                    (-1, 0),
                    (0, -1),
                    (1, 0),
                    (0, 1),
                ]) {
                    if s[ni][nj] == '#' {
                        adj[i][j] += 1;
                    }
                }
            }
        }
    }

    let mut que = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if adj[i][j] == 1 {
                que.push((i, j));
            }
        }
    }
    while !que.is_empty() {
        let mut new_que = Vec::new();
        for &(i, j) in &que {
            s[i][j] = '#';
        }
        for &(i, j) in &que {
            for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&[
                (-1, 0),
                (0, -1),
                (1, 0),
                (0, 1),
            ]) {
                if s[ni][nj] == '.' {
                    adj[ni][nj] += 1;
                    new_que.push((ni, nj));
                }
            }
        }
        new_que.retain(|&(ni, nj)| adj[ni][nj] == 1);
        que = new_que;
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

// Bundled
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
