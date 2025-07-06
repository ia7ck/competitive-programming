use std::collections::VecDeque;

use grid_search::around;
use proconio::{input, marker::Usize1};
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        ban: [(Usize1, Usize1); k],
    };

    let mut row = vec![vec![]; h];
    let mut col = vec![vec![]; w];

    // ban
    for &(y, x) in &ban {
        row[y].push(x);
        col[x].push(y);
    }

    // safe
    for &(r, c) in &ban {
        for (y, x) in around(r, c).y_range(0..h).x_range(0..w).directions(&[
            (-1, 0),
            // (-1, -1),
            (0, -1),
            // (1, -1),
            (1, 0),
            // (1, 1),
            (0, 1),
            // (-1, 1),
        ]) {
            row[y].push(x);
            col[x].push(y);
        }
    }
    for y in 0..h {
        for x in [0, w - 1] {
            row[y].push(x);
            col[x].push(y);
        }
    }
    for y in [0, h - 1] {
        for x in 0..w {
            row[y].push(x);
            col[x].push(y);
        }
    }

    let mut left = vec![FxHashMap::default(); h];
    let mut right = vec![FxHashMap::default(); h];
    let mut up = vec![FxHashMap::default(); w];
    let mut down = vec![FxHashMap::default(); w];
    for i in 0..h {
        row[i].sort_unstable();
        row[i].dedup();

        for w in row[i].windows(2) {
            left[i].insert(w[1], w[0]);
            right[i].insert(w[0], w[1]);
        }
    }
    for j in 0..w {
        col[j].sort_unstable();
        col[j].dedup();

        for w in col[j].windows(2) {
            up[j].insert(w[1], w[0]);
            down[j].insert(w[0], w[1]);
        }
    }

    let ban = ban.into_iter().collect::<FxHashSet<_>>();

    // BFS (0, 0) -> (h-1, w-1)
    let mut seen = FxHashSet::default();
    let mut que = VecDeque::new();
    seen.insert((0, 0));
    que.push_back((0, 0));
    while let Some((y, x)) = que.pop_front() {
        if let Some(&nx) = left[y].get(&x) {
            if !ban.contains(&(y, nx)) && seen.insert((y, nx)) {
                que.push_back((y, nx));
            }
        }
        if let Some(&nx) = right[y].get(&x) {
            if !ban.contains(&(y, nx)) && seen.insert((y, nx)) {
                que.push_back((y, nx));
            }
        }
        if let Some(&ny) = up[x].get(&y) {
            if !ban.contains(&(ny, x)) && seen.insert((ny, x)) {
                que.push_back((ny, x));
            }
        }
        if let Some(&ny) = down[x].get(&y) {
            if !ban.contains(&(ny, x)) && seen.insert((ny, x)) {
                que.push_back((ny, x));
            }
        }
    }

    if seen.contains(&(h - 1, w - 1)) {
        println!("Yes");
    } else {
        println!("No");
    }
}

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
