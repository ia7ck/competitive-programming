use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    const INF: usize = usize::MAX / 3;
    let mut d = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'E' {
                d[i][j] = 0;
                que.push_back((i, j));
            }
        }
    }
    while let Some((i, j)) = que.pop_front() {
        for (ni, nj) in grid_search::around(i, j)
            .y_range(0..h)
            .x_range(0..w)
            .directions(&[(-1, 0), (1, 0), (0, -1), (0, 1)])
        {
            if s[ni][nj] == '#' {
                continue;
            }
            if d[ni][nj] == INF {
                d[ni][nj] = d[i][j] + 1;
                que.push_back((ni, nj));
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            let c = if s[i][j] == 'E' || s[i][j] == '#' {
                s[i][j]
            } else {
                assert_ne!(d[i][j], INF);
                assert_ne!(d[i][j], 0);
                let (ni, nj) = grid_search::around(i, j)
                    .y_range(0..h)
                    .x_range(0..w)
                    .directions(&[(-1, 0), (1, 0), (0, -1), (0, 1)])
                    .find(|&(ni, nj)| d[ni][nj] == d[i][j] - 1)
                    .unwrap();
                if ni < i {
                    '^'
                } else if ni > i {
                    'v'
                } else if nj < j {
                    '<'
                } else if nj > j {
                    '>'
                } else {
                    unreachable!()
                }
            };
            print!("{}", c);
        }
        println!();
    }
}
