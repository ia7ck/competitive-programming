use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, usize); m],
    };

    let mut g = vec![vec![]; n];
    for &(u, v, w) in &edges {
        g[u].push((v, w));
    }
    let mut que = VecDeque::new();
    let mut visit = vec![[false; 1 << 10]; n];
    que.push_back((0, 0));
    visit[0][0] = true;
    while let Some((x, state)) = que.pop_front() {
        for &(y, w) in &g[x] {
            let new_state = state ^ w;
            if !visit[y][new_state] {
                visit[y][new_state] = true;
                que.push_back((y, new_state));
            }
        }
    }

    let ans = (0..(1 << 10)).find(|&i| visit[n - 1][i]);
    match ans {
        Some(ans) => {
            println!("{}", ans);
        }
        None => {
            println!("-1");
        }
    }
}
