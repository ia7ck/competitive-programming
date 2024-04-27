use std::collections::{HashSet, VecDeque};

use grid_search::around;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut t = s.clone();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&[
                    (0, 1),
                    (-1, 0),
                    (0, -1),
                    (1, 0),
                ]) {
                    t[ni][nj] = '#';
                }
            }
        }
    }

    let mut ans = 1;
    let mut seen = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if t[i][j] == '#' {
                continue;
            }
            if seen[i][j] {
                continue;
            }
            let mut interior = Vec::new();
            let mut que = VecDeque::new();
            seen[i][j] = true;
            que.push_back((i, j));
            while let Some((i, j)) = que.pop_front() {
                assert!(seen[i][j]);
                assert_eq!(t[i][j], '.');
                interior.push((i, j));
                for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&[
                    (0, 1),
                    (-1, 0),
                    (0, -1),
                    (1, 0),
                ]) {
                    if t[ni][nj] == '#' {
                        continue;
                    }
                    if seen[ni][nj] {
                        continue;
                    }
                    seen[ni][nj] = true;
                    que.push_back((ni, nj));
                }
            }
            let mut terminate = HashSet::new();
            for &(i, j) in &interior {
                for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&[
                    (0, 1),
                    (-1, 0),
                    (0, -1),
                    (1, 0),
                ]) {
                    if t[ni][nj] == '#' {
                        terminate.insert((ni, nj));
                    }
                }
            }
            ans = ans.max(interior.len() + terminate.len());
        }
    }
    println!("{}", ans);
}

// 3 5
// ###..
// .#..#
// #####
