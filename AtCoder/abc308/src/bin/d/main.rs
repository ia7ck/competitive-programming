use std::collections::VecDeque;

use proconio::{input, marker::Chars};
use grid_search::around;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let next = |ch: char| -> char {
        match ch {
            's' => 'n',
            'n' => 'u',
            'u' => 'k',
            'k' => 'e',
            'e' => 's',
            _ => unreachable!(),
        }
    };

    let mut que = VecDeque::new();
    let mut seen = vec![vec![false; w]; h];
    if s[0][0] == 's' {
        seen[0][0] = true;
        que.push_back((0, 0));
    }
    let dirs = &[(-1, 0), (0, -1), (1, 0), (0, 1)];
    while let Some((i, j)) = que.pop_front() {
        let nc = next(s[i][j]);
        for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(dirs) {
            if s[ni][nj] == nc && seen[ni][nj] == false {
                seen[ni][nj] = true;
                que.push_back((ni, nj));
            }
        }
    }

    if seen[h - 1][w - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
