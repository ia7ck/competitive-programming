use std::collections::VecDeque;

use proconio::{input, marker::Bytes};

const INF: u64 = std::u64::MAX;

fn f(n: usize, g: &Vec<Vec<usize>>, s: usize) -> Vec<u64> {
    let mut dist = vec![INF; n];
    dist[s] = 0;
    let mut que = VecDeque::new();
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in &g[u] {
            if dist[v] == INF {
                dist[v] = dist[u] + 1;
                que.push_back(v);
            }
        }
    }
    dist
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Bytes; n],
    };

    let mut edges = Vec::new();
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == b'1' {
                assert!(i + j + 1 < n);
                edges.push((i, i + j + 1));
            }
        }
    }
    let mut g = vec![vec![]; n];
    let mut rev_g = vec![vec![]; n];
    for &(s, t) in &edges {
        g[s].push(t);
        rev_g[t].push(s);
    }

    let dist = f(n, &g, 0);
    let rev_dist = f(n, &rev_g, n - 1);

    let mut ans = Vec::new();
    // N
    for k in 1..(n - 1) {
        let mut d = INF;
        // M
        for i in k.saturating_sub(m)..k {
            // M
            for j in (k + 1)..(k + m).min(n) {
                if j - i - 1 >= m {
                    continue;
                }
                if s[i][j - i - 1] == b'1' {
                    d = d.min(dist[i].saturating_add(rev_dist[j]).saturating_add(1));
                }
            }
        }
        if d == INF {
            ans.push(-1);
        } else {
            ans.push(d as i64);
        }
    }

    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i + 1 < ans.len() {
            print!(" ");
        } else {
            println!();
        }
    }
}
