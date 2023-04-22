use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

const INF: usize = std::usize::MAX;

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
        edges: [(Usize1, Usize1); m],
        k: usize,
        pd: [(Usize1, usize); k],
    };

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut black = vec![true; n];
    for &(p, d) in &pd {
        let dist = bfs(n, p, &g);
        for v in 0..n {
            if dist[v] < d {
                black[v] = false;
            }
        }
    }

    for &(p, d) in &pd {
        let dist = bfs(n, p, &g);
        let mut f = INF;
        for v in 0..n {
            if black[v] {
                f = f.min(dist[v]);
            }
        }
        if d != f {
            println!("No");
            return;
        }
    }

    println!("Yes");
    for b in black {
        print!("{}", b as u8);
    }
    print!("\n");
}
