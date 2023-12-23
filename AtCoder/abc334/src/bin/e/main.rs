use mod_int::ModInt998244353;
use proconio::{input, marker::Chars};

fn dfs(c: usize, i: usize, j: usize, s: &Vec<Vec<char>>, visited: &mut Vec<Vec<usize>>) {
    if visited[i][j] != 0 {
        return;
    }
    visited[i][j] = c;
    if i >= 1 && s[i - 1][j] == '#' {
        dfs(c, i - 1, j, s, visited);
    }
    if i + 1 < s.len() && s[i + 1][j] == '#' {
        dfs(c, i + 1, j, s, visited);
    }
    if j >= 1 && s[i][j - 1] == '#' {
        dfs(c, i, j - 1, s, visited);
    }
    if j + 1 < s[i].len() && s[i][j + 1] == '#' {
        dfs(c, i, j + 1, s, visited);
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h], // '.' : red, '#' : green
    };

    type Mint = ModInt998244353;

    let mut visited = vec![vec![0; w]; h];
    let mut conn = 0;
    for i in 0..h {
        for j in 0..w {
            if visited[i][j] == 0 && s[i][j] == '#' {
                conn += 1;
                dfs(conn, i, j, &s, &mut visited);
            }
        }
    }

    let mut red = 0;
    let mut ans = Mint::new(0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            red += 1;
            let mut v = Vec::new();
            if i >= 1 && s[i - 1][j] == '#' {
                v.push(visited[i - 1][j]);
            }
            if i + 1 < h && s[i + 1][j] == '#' {
                v.push(visited[i + 1][j]);
            }
            if j >= 1 && s[i][j - 1] == '#' {
                v.push(visited[i][j - 1]);
            }
            if j + 1 < w && s[i][j + 1] == '#' {
                v.push(visited[i][j + 1]);
            }
            v.sort();
            v.dedup();
            match v.len() {
                0 => {ans += conn + 1;},
                1 => {
                    ans += conn;
                }
                2 => {
                    assert!(conn >= 1);
                    ans += conn - 1;
                },
                3 => {
                    assert!(conn >= 2);
                    ans += conn - 2;
                },
                4 => {
                    assert!(conn >= 3);
                    ans += conn - 3;
                },
                _ => unreachable!(),
            }
        }
    }

    let ans = ans / red;
    println!("{}", ans.val());
}
