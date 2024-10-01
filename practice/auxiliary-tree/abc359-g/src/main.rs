use std::collections::HashMap;

use auxiliary_tree::auxiliary_tree;
use graph::tree_drop_parent;
use lowest_common_ancestor::LowestCommonAncestor;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        a: [Usize1; n],
    };

    let mut nodes_by_color = vec![vec![]; n];
    for i in 0..n {
        nodes_by_color[a[i]].push(i);
    }

    let (g, _parent) = tree_drop_parent(n, 0, &edges);
    let mut ord = Vec::new();
    dfs(0, &g, &mut ord);
    let mut inv_ord = vec![0; n];
    for i in 0..n {
        inv_ord[ord[i]] = i;
    }

    let lca = LowestCommonAncestor::new(n, 0, &edges);

    let mut ans = 0;
    let mut count = vec![0; n]; // 使い回し
    for (color, nodes) in nodes_by_color.iter().enumerate() {
        if nodes.is_empty() {
            continue;
        }
        let (root, h) = auxiliary_tree(&nodes, &inv_ord, &lca);
        solve(root, color, &a, &h, &lca, &mut count);
        for (i, children) in h {
            for j in children {
                // 辺 (i, j) の寄与を足す
                ans += count[j] * (nodes.len() - count[j]) * lca.get_dist(i, j);
            }
        }
    }

    println!("{}", ans);
}

fn dfs(i: usize, g: &Vec<Vec<usize>>, ord: &mut Vec<usize>) {
    ord.push(i);
    for &j in &g[i] {
        dfs(j, g, ord)
    }
}

fn solve(
    i: usize,
    color: usize,
    a: &Vec<usize>,
    g: &HashMap<usize, Vec<usize>>,
    lca: &LowestCommonAncestor,
    count: &mut Vec<usize>,
) {
    count[i] = usize::from(a[i] == color);
    for &j in &g[&i] {
        solve(j, color, a, g, lca, count);
        count[i] += count[j];
    }
}
