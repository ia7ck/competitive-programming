use std::{collections::VecDeque, println};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    let mut seen = vec![vec![false; m]; n];
    seen[1][1] = true;
    let mut que = VecDeque::new();
    que.push_back((1, 1));
    let mut pass = vec![vec![false; m]; n];
    while let Some((i, j)) = que.pop_front() {
        let up = {
            let mut y = i;
            while s[y][j] == '.' {
                pass[y][j] = true;
                y -= 1;
            }
            y + 1
        };
        let down = {
            let mut y = i;
            while s[y][j] == '.' {
                pass[y][j] = true;
                y += 1;
            }
            y - 1
        };
        let left = {
            let mut x = j;
            while s[i][x] == '.' {
                pass[i][x] = true;
                x -= 1;
            }
            x + 1
        };
        let right = {
            let mut x = j;
            while s[i][x] == '.' {
                pass[i][x] = true;
                x += 1;
            }
            x - 1
        };
        for &(ni, nj) in &[(up, j), (down, j), (i, left), (i, right)] {
            if seen[ni][nj] == false {
                seen[ni][nj] = true;
                que.push_back((ni, nj));
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if seen[i][j] || pass[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
