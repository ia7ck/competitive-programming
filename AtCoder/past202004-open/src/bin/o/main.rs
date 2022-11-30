use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn dfs(
    i: usize,
    p: usize,
    g: &Vec<Vec<(usize, u64)>>,
    parent: &mut Vec<usize>,
    depth: &mut Vec<usize>,
    weight: &mut Vec<u64>,
) {
    parent[i] = p;
    for &(j, c) in &g[i] {
        if j == p {
            continue;
        }
        depth[j] = depth[i] + 1;
        weight[j] = c;
        dfs(j, i, g, parent, depth, weight);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut edges = Vec::new();
    for i in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            c: u64,
        };
        edges.push((a, b, c, i));
    }

    let mut uf = UnionFind::new(n);
    edges.sort_by_key(|&(_, _, c, _)| c);
    let mut mst_cost = 0;
    let mut mst_edge = vec![false; m];
    for &(a, b, c, i) in &edges {
        if uf.same(a, b) {
            continue;
        }
        uf.unite(a, b);
        mst_cost += c;
        mst_edge[i] = true;
    }

    let mut g = vec![vec![]; n];
    for &(a, b, c, i) in &edges {
        if mst_edge[i] {
            g[a].push((b, c));
            g[b].push((a, c));
        }
    }

    const ILLEGAL: usize = std::usize::MAX;
    let mut parent = vec![ILLEGAL; n];
    let mut depth = vec![0; n];
    let mut weight = vec![0; n]; // weight[i] := 辺 parent[i] -> i の重み
    dfs(0, ILLEGAL, &g, &mut parent, &mut depth, &mut weight);

    const L: usize = 17; // 2^17 >= 1e5
    let mut parent_lg = vec![vec![ILLEGAL; n]; L];
    let mut max_weight_lg = vec![vec![0; n]; L];
    parent_lg[0] = parent;
    max_weight_lg[0] = weight;
    for i in 1..L {
        for v in 0..n {
            let p = parent_lg[i - 1][v];
            if p == ILLEGAL {
                continue;
            }
            parent_lg[i][v] = parent_lg[i - 1][p];
            max_weight_lg[i][v] = max_weight_lg[i - 1][v].max(max_weight_lg[i - 1][p]);
        }
    }

    edges.sort_by_key(|&(_, _, _, i)| i);
    for &(a, b, c, i) in &edges {
        if mst_edge[i] {
            println!("{}", mst_cost);
        } else {
            // サイクル a -- b -- LCA -- a 内の辺の重みの最大値を求める

            // LCA を求める
            let lca = {
                let (mut a, b) = if depth[a] >= depth[b] { (a, b) } else { (b, a) };
                // 同じ深さまで登る
                for i in 0..L {
                    if (depth[a] - depth[b]) >> i & 1 == 1 {
                        a = parent_lg[i][a];
                        assert_ne!(a, ILLEGAL);
                    }
                }
                let mut b = b;
                if a == b {
                    a
                } else {
                    // LCA 直下まで登る
                    for i in (0..L).rev() {
                        if parent_lg[i][a] != parent_lg[i][b] {
                            a = parent_lg[i][a];
                            b = parent_lg[i][b];
                        }
                    }
                    assert_eq!(parent_lg[0][a], parent_lg[0][b]);
                    // 1だけ登る
                    parent_lg[0][a]
                }
            };

            let w = [a, b]
                .iter()
                .copied()
                .map(|mut u| {
                    let mut w = 0;
                    for i in 0..L {
                        if (depth[u] - depth[lca]) >> i & 1 == 1 {
                            w = w.max(max_weight_lg[i][u]);
                            u = parent_lg[i][u];
                        }
                    }
                    w
                })
                .max()
                .unwrap();
            println!("{}", mst_cost + c - w);
        }
    }
}
