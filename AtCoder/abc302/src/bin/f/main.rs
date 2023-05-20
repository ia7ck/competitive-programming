use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

const INF: usize = std::usize::MAX / 2;
fn bfs(n: usize, s: usize, g: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut d = vec![INF; n];
    let mut que = VecDeque::new();
    d[s] = 0;
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in &g[u] {
            if d[v] == INF {
                d[v] = d[u] + 1;
                que.push_back(v);
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [[Usize1]; n],
    };

    let mut g = vec![vec![]; n + m];
    for i in 0..n {
        for &x in &s[i] {
            g[i].push(n + x);
            g[n + x].push(i);
        }
    }
    let d = bfs(n + m, n + 0, &g);
    let ans = d[n + (m - 1)];
    if ans == INF {
        println!("-1");
    } else {
        assert!(ans >= 2);
        assert_eq!(ans % 2, 0);
        println!("{}", ans / 2 - 1);
    }
}
