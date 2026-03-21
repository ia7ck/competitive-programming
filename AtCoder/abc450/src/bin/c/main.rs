use std::collections::HashSet;

use proconio::{input, marker::Chars};

use crate::grid_search::around;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut ans = 0;
    let mut seen = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            if seen[i][j] {
                continue;
            }
            let mut cur = HashSet::new();
            dfs(i, j, &s, &mut cur);
            let mut ok = true;
            for (i, j) in cur {
                seen[i][j] = true;
                if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                    ok = false;
                }
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn dfs(i: usize, j: usize, s: &Vec<Vec<char>>, seen: &mut HashSet<(usize, usize)>) {
    if s[i][j] == '#' {
        return;
    }

    if !seen.insert((i, j)) {
        return;
    }

    for (ni, nj) in around(i, j)
        .y_range(0..s.len())
        .x_range(0..s[i].len())
        .directions(&[(-1, 0), (0, -1), (1, 0), (0, 1)])
    {
        dfs(ni, nj, s, seen);
    }
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
