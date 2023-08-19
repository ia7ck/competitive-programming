use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    };

    let mut g = vec![vec![]; n];
    for i in 0..n {
        input! {
            c: usize,
            p: [Usize1; c],
        };
        g[i] = p;
    }

    let mut visited = vec![false; n];
    let mut que = VecDeque::new();
    que.push_back(0);
    while let Some(i) = que.pop_front() {
        for &j in &g[i] {
            if visited[j] {
                continue;
            }
            visited[j] = true;
            que.push_back(j);
        }
    }

    let mut deg = vec![0; n];
    let mut rg = vec![vec![]; n];
    for i in 0..n {
        deg[i] = g[i].len();
        for &j in &g[i] {
            rg[j].push(i);
        }
    }

    let mut que = VecDeque::new();
    for i in 0..n {
        if visited[i] && deg[i] == 0 {
            que.push_back(i);
        }
    }
    let mut ans = Vec::new();
    while let Some(i) = que.pop_front() {
        if i == 0 {
            break;
        }
        ans.push(i);
        for &j in &rg[i] {
            assert!(deg[j] >= 1);
            deg[j] -= 1;
            if visited[j] && deg[j] == 0 {
                que.push_back(j);
            }
        }
    }
    let ans = ans
        .into_iter()
        .map(|i| i + 1)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}
