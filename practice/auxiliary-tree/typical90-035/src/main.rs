use std::collections::HashMap;

use lowest_common_ancestor::LowestCommonAncestor;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        q: usize,
        queries: [[Usize1]; q],
    };

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }
    let mut ord = Vec::new();
    let mut parent = vec![usize::MAX; n];
    dfs(0, usize::MAX, &g, &mut ord, &mut parent);

    let mut inv_ord = vec![0; n];
    for i in 0..n {
        inv_ord[ord[i]] = i;
    }
    let lca = LowestCommonAncestor::new(n, 0, &edges);

    for nodes in queries {
        let h = auxiliary_tree(&nodes, &inv_ord, &lca);
        let mut ans = 0;
        for (i, children) in h {
            for j in children {
                ans += lca.get_dist(i, j);
            }
        }
        println!("{}", ans);
    }
}

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, ord: &mut Vec<usize>, parent: &mut Vec<usize>) {
    ord.push(i);
    parent[i] = p;
    for &j in g[i].iter() {
        if j != p {
            dfs(j, i, g, ord, parent);
        }
    }
}

fn auxiliary_tree(
    nodes: &Vec<usize>,
    inv_ord: &Vec<usize>,
    lca: &LowestCommonAncestor,
) -> HashMap<usize, Vec<usize>> {
    if nodes.is_empty() {
        return HashMap::new();
    }
    if nodes.len() == 1 {
        return HashMap::from([(nodes[0], vec![])]);
    }

    let mut nodes = nodes.clone();
    nodes.sort_by_key(|&i| inv_ord[i]);

    let lca_nodes = nodes
        .windows(2)
        .map(|w| lca.get(w[0], w[1]))
        .collect::<Vec<_>>();
    nodes.extend(lca_nodes);
    nodes.sort_by_key(|&i| inv_ord[i]);
    nodes.dedup();

    let mut h = HashMap::<_, Vec<_>>::new();
    let mut root_path = Vec::new();
    root_path.push(nodes[0]);
    for w in nodes.windows(2) {
        assert_eq!(root_path.last(), Some(&w[0]));

        let x = lca.get(w[0], w[1]);
        while let Some(&y) = root_path.last() {
            if lca.depth(x) < lca.depth(y) {
                root_path.pop(); // y
            } else {
                assert_eq!(lca.depth(x), lca.depth(y));
                assert_eq!(x, y);
                break;
            }
        }

        root_path.push(w[1]);
        h.entry(x).or_insert_with(Vec::new).push(w[1]);

        assert!(!h.contains_key(&w[1]));
        h.insert(w[1], vec![]);
    }
    h
}
