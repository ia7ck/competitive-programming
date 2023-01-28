use graph::is_tree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    if !is_tree(n, &edges) {
        println!("No");
        return;
    }

    let mut deg = vec![0; n];
    for (u, v) in edges {
        deg[u] += 1;
        deg[v] += 1;
    }

    if deg.iter().all(|&d| d <= 2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
