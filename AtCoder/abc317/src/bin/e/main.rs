use std::collections::VecDeque;

use grid_search::around;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };

    let mut up = vec![vec![false; w]; h];
    let mut down = vec![vec![false; w]; h];
    for j in 0..w {
        for i in 1..h {
            if a[i][j] == '.' {
                up[i][j] = up[i - 1][j] || a[i - 1][j] == 'v';
            }
        }
        for i in (0..(h - 1)).rev() {
            if a[i][j] == '.' {
                down[i][j] = down[i + 1][j] || a[i + 1][j] == '^';
            }
        }
    }
    let mut left = vec![vec![false; w]; h];
    let mut right = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 1..w {
            if a[i][j] == '.' {
                left[i][j] = left[i][j - 1] || a[i][j - 1] == '>';
            }
        }
        for j in (0..(w - 1)).rev() {
            if a[i][j] == '.' {
                right[i][j] = right[i][j + 1] || a[i][j + 1] == '<';
            }
        }
    }

    // for i in 0..h {
    //     for j in 0..w {
    //         if up[i][j] || down[i][j] || left[i][j] || right[i][j] {
    //             eprint!("!");
    //         } else {
    //             eprint!("{}", a[i][j]);
    //         }
    //     }
    //     eprintln!();
    // }

    let (mut si, mut sj) = (0, 0);
    let (mut gi, mut gj) = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                si = i;
                sj = j;
            }
            if a[i][j] == 'G' {
                gi = i;
                gj = j;
            }
        }
    }
    const INF: usize = usize::MAX / 2;
    let mut dist = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();
    dist[si][sj] = 0;
    que.push_back((si, sj));
    let dirs = &[(-1, 0), (0, -1), (1, 0), (0, 1)];
    while let Some((i, j)) = que.pop_front() {
        for (y, x) in around(i, j).y_range(0..h).x_range(0..w).directions(dirs) {
            if up[y][x] || down[y][x] || left[y][x] || right[y][x] {
                continue;
            }
            if a[y][x] == '.' || a[y][x] == 'G' {
                if dist[y][x] == INF {
                    dist[y][x] = dist[i][j] + 1;
                    que.push_back((y, x));
                }
            }
        }
    }

    let ans = dist[gi][gj];
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
