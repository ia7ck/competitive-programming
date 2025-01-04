use std::collections::VecDeque;

use grid_search::around;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    const INF: usize = usize::MAX / 2;
    let mut dist = vec![vec![vec![INF; w]; h]; 2];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                for dir in 0..2 {
                    dist[dir][i][j] = 0;
                    que.push_back((dir, i, j));
                }
            }
        }
    }
    while let Some((dir, i, j)) = que.pop_front() {
        for (ni, nj) in around(i, j)
            .y_range(0..h)
            .x_range(0..w)
            .directions(if dir == 0 {
                &[(-1, 0), (1, 0)]
            } else {
                &[(0, -1), (0, 1)]
            })
        {
            if s[ni][nj] == '#' {
                continue;
            }
            let nd = dist[dir][i][j] + 1;
            if nd < dist[dir ^ 1][ni][nj] {
                dist[dir ^ 1][ni][nj] = nd;
                que.push_back((dir ^ 1, ni, nj));
            }
        }
    }

    let mut ans = INF;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'G' {
                for dir in 0..2 {
                    ans = ans.min(dist[dir][i][j]);
                }
            }
        }
    }
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
