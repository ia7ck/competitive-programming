use grid_search::around;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    };

    let mut ans = 0_usize;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut seen = vec![vec![false; w]; h];
            seen[i][j] = true;
            ans += dfs((i, j), k, &s, &mut seen);
        }
    }

    println!("{}", ans);
}

fn dfs((i, j): (usize, usize), k: usize, s: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>) -> usize {
    if k == 0 {
        return 1;
    }

    let mut ans = 0;
    for (ni, nj) in around(i, j)
        .y_range(0..s.len())
        .x_range(0..s[i].len())
        .directions(&[(-1, 0), (0, -1), (1, 0), (0, 1)])
    {
        if seen[ni][nj] {
            continue;
        }
        if s[ni][nj] == '#' {
            continue;
        }
        seen[ni][nj] = true;
        ans += dfs((ni, nj), k - 1, s, seen);
        seen[ni][nj] = false;
    }

    ans
}
