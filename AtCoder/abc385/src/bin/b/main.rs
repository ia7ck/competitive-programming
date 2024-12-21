use std::collections::HashSet;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        s: [Chars; h],
        t: Chars,
    };

    let mut visited = HashSet::new();
    visited.insert((x, y));
    let (mut i, mut j) = (x, y);
    for c in t {
        let (ni, nj) = match c {
            'U' => {
                if i >= 1 {
                    (i - 1, j)
                } else {
                    (i, j)
                }
            }
            'D' => {
                if i + 1 < h {
                    (i + 1, j)
                } else {
                    (i, j)
                }
            }
            'L' => {
                if j >= 1 {
                    (i, j - 1)
                } else {
                    (i, j)
                }
            }
            'R' => {
                if j + 1 < w {
                    (i, j + 1)
                } else {
                    (i, j)
                }
            }
            _ => unreachable!(),
        };
        if s[ni][nj] == '#' {
            continue;
        }
        visited.insert((ni, nj));
        (i, j) = (ni, nj);
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '@' && visited.contains(&(i, j)) {
                ans += 1;
            }
        }
    }
    println!("{} {} {}", i + 1, j + 1, ans);
}
