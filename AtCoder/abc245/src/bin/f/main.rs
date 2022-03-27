use scanner_proc_macro::insert_scanner;
use std::collections::VecDeque;
use strongly_connected_components::strongly_connected_components;

#[insert_scanner]
fn main() {
    let (n, m) = scan!((usize, usize));

    let mut edges = Vec::new();
    for _ in 0..m {
        let (u, v) = scan!((usize, usize));
        edges.push((u - 1, v - 1));
    }
    let scc = strongly_connected_components(n, edges.iter().copied());

    let mut index = vec![0; n];
    for i in 0..scc.len() {
        for &j in &scc[i] {
            index[j] = i;
        }
    }

    let mut rev_g = vec![vec![]; scc.len()];
    let mut deg = vec![0; scc.len()];
    for &(u, v) in &edges {
        let u = index[u];
        let v = index[v];
        if u == v {
            continue;
        }
        rev_g[v].push(u);
        deg[u] += 1;
    }

    let mut dp = vec![false; rev_g.len()];
    let mut que = VecDeque::new();
    for u in 0..rev_g.len() {
        dp[u] = scc[u].len() >= 2;
        if deg[u] == 0 {
            que.push_back(u);
        }
    }
    while let Some(u) = que.pop_front() {
        for &v in &rev_g[u] {
            dp[v] |= dp[u];
            deg[v] -= 1;
            if deg[v] == 0 {
                que.push_back(v);
            }
        }
    }
    let mut ans = 0;
    for u in 0..n {
        if dp[index[u]] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
