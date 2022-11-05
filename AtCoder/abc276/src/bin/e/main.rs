use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let (mut si, mut sj) = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                si = i;
                sj = j;
            }
        }
    }

    let mut adj = Vec::new();
    if si >= 1 && c[si - 1][sj] == '.' {
        adj.push((si - 1, sj));
    }
    if sj >= 1 && c[si][sj - 1] == '.' {
        adj.push((si, sj - 1));
    }
    if si + 1 < h && c[si + 1][sj] == '.' {
        adj.push((si + 1, sj));
    }
    if sj + 1 < w && c[si][sj + 1] == '.' {
        adj.push((si, sj + 1));
    }

    for &(ai, aj) in &adj {
        let mut visited = vec![vec![false; w]; h];
        let mut que = VecDeque::new();
        visited[ai][aj] = true;
        que.push_back((ai, aj));
        while let Some((i, j)) = que.pop_front() {
            if i >= 1 && c[i - 1][j] == '.' && !visited[i - 1][j] {
                visited[i - 1][j] = true;
                que.push_back((i - 1, j));
            }
            if j >= 1 && c[i][j - 1] == '.' && !visited[i][j - 1] {
                visited[i][j - 1] = true;
                que.push_back((i, j - 1));
            }
            if i + 1 < h && c[i + 1][j] == '.' && !visited[i + 1][j] {
                visited[i + 1][j] = true;
                que.push_back((i + 1, j));
            }
            if j + 1 < w && c[i][j + 1] == '.' && !visited[i][j + 1] {
                visited[i][j + 1] = true;
                que.push_back((i, j + 1));
            }
        }
        for &(oi, oj) in &adj {
            if (oi, oj) != (ai, aj) && visited[oi][oj] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
