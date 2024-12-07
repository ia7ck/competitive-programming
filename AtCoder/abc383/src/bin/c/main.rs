use std::collections::VecDeque;

use grid_search::around;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    };

    let mut dist = vec![vec![usize::MAX; w]; h];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
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
            if s[ni][nj] == '.' && dist[ni][nj] == usize::MAX {
                dist[ni][nj] = dist[i][j] + 1;
                que.push_back((ni, nj));
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if dist[i][j] <= d {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
