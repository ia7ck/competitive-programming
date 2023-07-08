use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

const INF: usize = std::usize::MAX / 2;

fn bfs(n: usize, s: usize, g: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut que = VecDeque::new();
    que.push_back(s);
    let mut dist = vec![INF; n];
    dist[s] = 0;
    while let Some(v) = que.pop_front() {
        for &x in &g[v] {
            if dist[x] == INF {
                dist[x] = dist[v] + 1;
                que.push_back(x);
            }
        }
    }
    dist
}

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut g1 = vec![vec![]; n1];
    let mut g2 = vec![vec![]; n2];
    for (a, b) in edges {
        if a < n1 {
            assert!(b < n1);
            g1[a].push(b);
            g1[b].push(a);
        } else {
            assert!(b >= n1);
            let a = a - n1;
            let b = b - n1;
            g2[a].push(b);
            g2[b].push(a);
        }
    }

    let d1 = bfs(n1, 0, &g1);
    let d2 = bfs(n2, n2 - 1, &g2);

    let d1 = d1.iter().max().copied().unwrap();
    assert_ne!(d1, INF);
    let d2 = d2.iter().max().copied().unwrap();
    assert_ne!(d2, INF);
    println!("{}", d1 + d2 + 1);

}
