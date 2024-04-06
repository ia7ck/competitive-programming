use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn dfs(
    i: usize,
    p: usize,
    g: &Vec<Vec<usize>>,
    weight: &Vec<usize>,
    dp_f: &mut Vec<usize>,
    dp_w: &mut Vec<usize>,
) {
    dp_f[i] = 0;
    dp_w[i] = weight[i];
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, weight, dp_f, dp_w);
        dp_w[i] += dp_w[j];
        dp_f[i] += dp_f[j] + dp_w[j];
    }
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        weight: [usize; n],
    };

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut dp_f = vec![0; n];
    let mut dp_w = vec![0; n];
    dfs(0, usize::MAX, &g, &weight, &mut dp_f, &mut dp_w);

    let mut f = vec![0; n];
    let mut que = VecDeque::new();
    f[0] = dp_f[0];
    que.push_back((0, usize::MAX));
    while let Some((i, p)) = que.pop_front() {
        for &j in &g[i] {
            if j != p {
                assert!(f[i] >= dp_f[j] + dp_w[j]);
                f[j] = dp_f[j] + (f[i] - dp_f[j] - dp_w[j]) + (dp_w[0] - dp_w[j]);
                que.push_back((j, i));
            }
        }
    }

    let ans = f.iter().min().unwrap();
    println!("{}", ans);
}
