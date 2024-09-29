use std::collections::HashMap;

use lowest_common_ancestor::LowestCommonAncestor;
use mod_int::ModInt998244353;
use proconio::{input, marker::Usize1};

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut nodes_by_color = vec![vec![]; n];
    for i in 0..n {
        nodes_by_color[a[i]].push(i);
    }

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }
    let mut ord = Vec::new();
    let mut parent = vec![0; n];
    dfs(0, usize::MAX, &g, &mut ord, &mut parent);
    let mut inv_ord = vec![0; n];
    for i in 0..n {
        inv_ord[ord[i]] = i;
    }
    let lca = LowestCommonAncestor::new(n, 0, &edges);

    let mut h = vec![HashMap::<_, Vec<_>>::new(); n];
    for (c, nodes) in nodes_by_color.iter().enumerate() {
        for (i, children) in auxiliary_tree(&nodes, &inv_ord, &lca) {
            h[i].entry(c).or_insert_with(Vec::new).extend(children);
        }
    }

    let mut ans = Mint::new(0);
    let mut dp = vec![HashMap::<usize, Mint>::new(); n];
    for &i in ord.iter().rev() {
        for (&c, children) in &h[i] {
            let mut prod = Mint::new(1);
            let mut sum = Mint::new(0);
            for &j in children {
                prod *= dp[j][&c] + 1;
                sum += dp[j][&c];
            }
            if a[i] == c {
                dp[i].insert(c, prod);
                ans += prod;
            } else {
                dp[i].insert(c, prod - 1);
                ans += prod - sum - 1;
            }
        }
    }

    println!("{}", ans.val());
}

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, ord: &mut Vec<usize>, parent: &mut Vec<usize>) {
    ord.push(i);
    parent[i] = p;
    for &j in &g[i] {
        if j != p {
            dfs(j, i, g, ord, parent);
        }
    }
}

// https://smijake3.hatenablog.com/entry/2019/09/15/200200
fn auxiliary_tree(
    nodes: &Vec<usize>,
    inv_ord: &Vec<usize>, // 頂点 i が行きがけ順で inv_ord[i] 番目に現われる
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

    let mut lca_nodes = Vec::new();
    for w in nodes.windows(2) {
        let x = lca.get(w[0], w[1]);
        lca_nodes.push(x);
    }
    nodes.extend(lca_nodes);
    nodes.sort();
    nodes.dedup();
    nodes.sort_by_key(|&i| inv_ord[i]);

    let mut h = HashMap::<_, Vec<_>>::new();
    let mut root_path = Vec::new(); // stack
    root_path.push(nodes[0]);
    for w in nodes.windows(2) {
        let (u, v) = (w[0], w[1]);
        assert_eq!(root_path.last(), Some(&u));

        let x = lca.get(u, v);
        while let Some(&y) = root_path.last() {
            if lca.depth(x) < lca.depth(y) {
                root_path.pop(); // y
            } else {
                assert_eq!(lca.depth(x), lca.depth(y)); // 合ってる？
                break;
            }
        }

        if let Some(&y) = root_path.last() {
            h.entry(y).or_insert_with(Vec::new).push(v);
        }
        root_path.push(v);
        assert!(!h.contains_key(&v));
        h.insert(v, vec![]);
    }

    h
}
