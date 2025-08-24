use std::collections::VecDeque;

use grid_search::around;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };

    const INF: usize = usize::MAX / 2;
    let mut dist = vec![vec![vec![INF; w]; h]; 2];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                dist[0][i][j] = 0;
                que.push_back((false, i, j));
            }
        }
    }

    while let Some((switch, i, j)) = que.pop_front() {
        for (ni, nj) in
            around(i, j)
                .y_range(0..h)
                .x_range(0..w)
                .directions(&[(-1, 0), (0, -1), (1, 0), (0, 1)])
        {
            if a[ni][nj] == '#' {
                continue;
            }
            if switch == false && a[ni][nj] == 'x' {
                continue;
            }
            if switch && a[ni][nj] == 'o' {
                continue;
            }
            let new_switch = if a[ni][nj] == '?' { !switch } else { switch };
            if dist[usize::from(new_switch)][ni][nj] < INF {
                continue;
            }
            dist[usize::from(new_switch)][ni][nj] = dist[usize::from(switch)][i][j] + 1;
            que.push_back((new_switch, ni, nj));
        }
    }

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'G' {
                let ans = dist[0][i][j].min(dist[1][i][j]);
                if ans == INF {
                    println!("-1");
                } else {
                    println!("{}", ans);
                }
                return;
            }
        }
    }

    unreachable!()
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
