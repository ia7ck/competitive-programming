use proconio::{input, marker::Chars};

fn dfs((i, j): (usize, usize), s: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
    if s[i][j] == '.' {
        return;
    }
    if visited[i][j] {
        return;
    }
    visited[i][j] = true;
    if i > 0 {
        dfs((i - 1, j), s, visited);
    }
    if i > 0 && j > 0 {
        dfs((i - 1, j - 1), s, visited);
    }
    if j > 0 {
        dfs((i, j - 1), s, visited);
    }
    if i + 1 < s.len() && j > 0 {
        dfs((i + 1, j - 1), s, visited);
    }
    if i + 1 < s.len() {
        dfs((i + 1, j), s, visited);
    }
    if i + 1 < s.len() && j + 1 < s[i].len() {
        dfs((i + 1, j + 1), s, visited);
    }
    if j + 1 < s[i].len() {
        dfs((i, j + 1), s, visited);
    }
    if i > 0 && j + 1 < s[i].len() {
        dfs((i - 1, j + 1), s, visited);
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut visited = vec![vec![false; w]; h];
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            if visited[i][j] {
                continue;
            }
            dfs((i, j), &s, &mut visited);
            ans += 1;
        }
    }
    println!("{}", ans);
}
